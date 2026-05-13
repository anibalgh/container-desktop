use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use domain::entities::{
    Image, ImageSecurityReport, SecurityFinding, SecurityImageSummary, SecurityImageToolStatus,
    SecurityInstallHint, SecurityOverview, SecurityReferenceLink, SecurityScanProgress,
    SecurityScanState, SecurityTool, SecurityToolReport, SecurityToolStatus, SeverityCount,
    VulnerabilitySeverity,
};
use domain::repository::{ImageRepository, SecurityRepository};
use domain::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;
use tokio::process::Command;
use tokio::sync::{Mutex, RwLock};
use tracing::warn;

use crate::DockerClient;

type ProgressCallback = Arc<dyn Fn(SecurityScanProgress) + Send + Sync>;
const REPORT_MAX_AGE_DAYS: i64 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredToolReport {
    image_id: String,
    image_name: String,
    tool: SecurityTool,
    tool_version: Option<String>,
    generated_at: String,
    findings: Vec<SecurityFinding>,
    severity_counts: Vec<SeverityCount>,
    raw_json: Value,
}

#[derive(Debug, Clone)]
struct RuntimeToolState {
    state: SecurityScanState,
    findings_count: Option<u32>,
    message: Option<String>,
    updated_at: String,
}

/// Coordinates security-tool detection, scanning, storage, and aggregation.
pub struct SecurityService {
    docker_client: Arc<DockerClient>,
    data_dir: PathBuf,
    active_tools: Arc<Mutex<BTreeSet<SecurityTool>>>,
    runtime_states: Arc<RwLock<HashMap<(String, SecurityTool), RuntimeToolState>>>,
}

impl SecurityService {
    /// Creates a new security service rooted in the given data directory.
    pub fn new(docker_client: Arc<DockerClient>, data_dir: PathBuf) -> Self {
        Self {
            docker_client,
            data_dir,
            active_tools: Arc::new(Mutex::new(BTreeSet::new())),
            runtime_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Detects scanner tools available on the current platform.
    pub async fn detect_tools(
        &self,
        selected_tools: &[SecurityTool],
    ) -> DomainResult<Vec<SecurityToolStatus>> {
        let selected: BTreeSet<SecurityTool> = selected_tools.iter().copied().collect();
        let mut statuses = Vec::new();

        for tool in SecurityTool::all() {
            let version = detect_tool_version(*tool).await;
            let available = version.is_some();
            statuses.push(SecurityToolStatus {
                tool: *tool,
                available,
                selected: available && selected.contains(tool),
                version,
                install_hint: install_hint_for(*tool),
            });
        }

        Ok(statuses)
    }

    /// Schedules background scans for the selected available tools.
    pub async fn schedule_scans(
        &self,
        selected_tools: Vec<SecurityTool>,
        notifier: ProgressCallback,
    ) -> DomainResult<()> {
        let images = self.docker_client.list_images().await?;
        let now = Utc::now();
        let results_dir = self.results_dir();
        let statuses = self.detect_tools(&selected_tools).await?;

        for status in statuses
            .into_iter()
            .filter(|status| status.selected && status.available)
        {
            let queue = build_scan_queue(&images, &results_dir, status.tool, now).await?;
            if queue.is_empty() {
                continue;
            }

            let should_spawn = {
                let mut active = self.active_tools.lock().await;
                active.insert(status.tool)
            };

            if !should_spawn {
                continue;
            }

            let data_dir = self.data_dir.clone();
            let runtime_states = self.runtime_states.clone();
            let active_tools = self.active_tools.clone();
            let notifier = notifier.clone();
            let tool = status.tool;
            let tool_version = status.version.clone();

            tokio::spawn(async move {
                if let Err(error) = run_tool_scan(
                    data_dir.join("results"),
                    runtime_states,
                    notifier,
                    tool,
                    tool_version,
                    queue,
                )
                .await
                {
                    warn!(
                        "Security scan worker for {} failed: {error}",
                        tool.display_name()
                    );
                }

                active_tools.lock().await.remove(&tool);
            });
        }

        Ok(())
    }

    fn results_dir(&self) -> PathBuf {
        self.data_dir.join("results")
    }
}

#[async_trait]
impl SecurityRepository for SecurityService {
    async fn security_overview(
        &self,
        selected_tools: &[SecurityTool],
    ) -> DomainResult<SecurityOverview> {
        fs::create_dir_all(self.results_dir())
            .await
            .map_err(|e| DomainError::Config(format!("Cannot create security results dir: {e}")))?;

        let images = self.docker_client.list_images().await?;
        let now = Utc::now();
        let selected_tool_set: BTreeSet<SecurityTool> = selected_tools.iter().copied().collect();
        let tool_statuses = self.detect_tools(selected_tools).await?;
        let selected_map: HashMap<SecurityTool, SecurityToolStatus> = tool_statuses
            .iter()
            .cloned()
            .map(|status| (status.tool, status))
            .collect();
        let runtime = self.runtime_states.read().await.clone();

        let mut findings_by_severity = zero_counts();
        let mut image_summaries = Vec::new();
        let mut scanned_images = 0_u32;
        let mut images_with_findings = 0_u32;

        for image in images {
            let stored_reports = read_reports_for_image(&self.results_dir(), &image.id).await?;
            let effective_reports =
                filter_effective_reports(&stored_reports, &selected_tool_set, now);
            let merged_findings = dedup_findings(&image.id, &effective_reports);
            let severity_counts = count_findings(merged_findings.values().cloned().collect());
            let total_findings = severity_counts.iter().map(|count| count.count).sum::<u32>();
            let tool_states = build_image_tool_states(
                &image,
                &selected_map,
                &stored_reports,
                &runtime,
                &selected_tool_set,
                now,
            );
            let last_scanned_at = tool_states
                .iter()
                .filter_map(|status| status.last_scanned_at.clone())
                .max();

            if tool_states.iter().any(|status| {
                status.selected && matches!(status.state, SecurityScanState::Completed)
            }) {
                scanned_images += 1;
            }

            if total_findings > 0 {
                images_with_findings += 1;
            }

            add_counts(&mut findings_by_severity, &severity_counts);

            image_summaries.push(SecurityImageSummary {
                image_id: image.id.clone(),
                image_name: image_reference(&image),
                repo_name: image.repo_name.clone(),
                tag: image.tag.clone(),
                total_findings,
                severity_counts,
                tool_statuses: tool_states,
                last_scanned_at,
            });
        }

        image_summaries.sort_by(|left, right| {
            right
                .total_findings
                .cmp(&left.total_findings)
                .then_with(|| left.image_name.cmp(&right.image_name))
        });

        Ok(SecurityOverview {
            tools: tool_statuses,
            total_images: image_summaries.len() as u32,
            scanned_images,
            images_with_findings,
            findings_by_severity,
            images: image_summaries,
        })
    }

    async fn image_security_report(&self, image_id: &str) -> DomainResult<ImageSecurityReport> {
        let images = self.docker_client.list_images().await?;
        let image = images
            .into_iter()
            .find(|image| image.id == image_id)
            .ok_or_else(|| DomainError::NotFound(format!("image {image_id}")))?;

        let stored_reports = read_reports_for_image(&self.results_dir(), image_id).await?;
        let now = Utc::now();
        let fresh_reports = filter_fresh_reports(&stored_reports, now);
        let runtime = self.runtime_states.read().await.clone();
        let mut reports = Vec::new();

        for tool in SecurityTool::all() {
            let stored = fresh_reports.iter().find(|report| report.tool == *tool);
            let runtime_state = runtime.get(&(image_id.to_string(), *tool));

            if let Some(report) = stored {
                let mut findings = report.findings.clone();
                sort_findings(&mut findings);
                reports.push(SecurityToolReport {
                    tool: report.tool,
                    state: runtime_state
                        .map(|state| state.state)
                        .unwrap_or(SecurityScanState::Completed),
                    tool_version: report.tool_version.clone(),
                    generated_at: Some(report.generated_at.clone()),
                    findings,
                    severity_counts: report.severity_counts.clone(),
                    message: runtime_state
                        .and_then(|state| state.message.clone())
                        .or_else(|| {
                            Some(format!(
                                "{} findings loaded from disk",
                                report.findings.len()
                            ))
                        }),
                });
            } else if let Some(state) = runtime_state {
                reports.push(SecurityToolReport {
                    tool: *tool,
                    state: state.state,
                    tool_version: None,
                    generated_at: Some(state.updated_at.clone()),
                    findings: Vec::new(),
                    severity_counts: zero_counts(),
                    message: state.message.clone(),
                });
            }
        }

        reports.sort_by_key(|report| report.tool);

        Ok(ImageSecurityReport {
            image_id: image.id.clone(),
            image_name: image_reference(&image),
            reports,
        })
    }
}

async fn run_tool_scan(
    results_dir: PathBuf,
    runtime_states: Arc<RwLock<HashMap<(String, SecurityTool), RuntimeToolState>>>,
    notifier: ProgressCallback,
    tool: SecurityTool,
    tool_version: Option<String>,
    queue: Vec<Image>,
) -> DomainResult<()> {
    fs::create_dir_all(&results_dir)
        .await
        .map_err(|e| DomainError::Config(format!("Cannot create security results dir: {e}")))?;

    for image in queue {
        let image_name = image_reference(&image);
        let started_at = now_rfc3339();

        update_runtime_state(
            &runtime_states,
            &image.id,
            tool,
            RuntimeToolState {
                state: SecurityScanState::Running,
                findings_count: None,
                message: Some(format!(
                    "Scanning {} with {}",
                    image_name,
                    tool.display_name()
                )),
                updated_at: started_at.clone(),
            },
        )
        .await;
        (notifier)(SecurityScanProgress {
            tool,
            image_id: image.id.clone(),
            image_name: image_name.clone(),
            state: SecurityScanState::Running,
            findings_count: None,
            message: Some(format!("Scanning {image_name}")),
        });

        match execute_tool_scan(tool, &image).await {
            Ok(raw_json) => {
                let findings = parse_findings(tool, &raw_json);
                let severity_counts = count_findings(findings.clone());
                let findings_count = findings.len() as u32;

                let report = StoredToolReport {
                    image_id: image.id.clone(),
                    image_name: image_name.clone(),
                    tool,
                    tool_version: tool_version.clone(),
                    generated_at: now_rfc3339(),
                    findings,
                    severity_counts: severity_counts.clone(),
                    raw_json,
                };

                if let Err(error) = write_report(&results_dir, &report).await {
                    let message = error.to_string();
                    update_runtime_state(
                        &runtime_states,
                        &image.id,
                        tool,
                        RuntimeToolState {
                            state: SecurityScanState::Failed,
                            findings_count: None,
                            message: Some(message.clone()),
                            updated_at: now_rfc3339(),
                        },
                    )
                    .await;
                    (notifier)(SecurityScanProgress {
                        tool,
                        image_id: image.id.clone(),
                        image_name: image_name.clone(),
                        state: SecurityScanState::Failed,
                        findings_count: None,
                        message: Some(message),
                    });
                    continue;
                }

                update_runtime_state(
                    &runtime_states,
                    &image.id,
                    tool,
                    RuntimeToolState {
                        state: SecurityScanState::Completed,
                        findings_count: Some(findings_count),
                        message: Some(format!("{findings_count} findings stored for {image_name}")),
                        updated_at: report.generated_at.clone(),
                    },
                )
                .await;
                (notifier)(SecurityScanProgress {
                    tool,
                    image_id: image.id.clone(),
                    image_name: image_name.clone(),
                    state: SecurityScanState::Completed,
                    findings_count: Some(findings_count),
                    message: Some(format!("Stored {findings_count} findings")),
                });
            }
            Err(error) => {
                let message = error.to_string();
                update_runtime_state(
                    &runtime_states,
                    &image.id,
                    tool,
                    RuntimeToolState {
                        state: SecurityScanState::Failed,
                        findings_count: None,
                        message: Some(message.clone()),
                        updated_at: now_rfc3339(),
                    },
                )
                .await;
                (notifier)(SecurityScanProgress {
                    tool,
                    image_id: image.id.clone(),
                    image_name: image_name.clone(),
                    state: SecurityScanState::Failed,
                    findings_count: None,
                    message: Some(message),
                });
            }
        }
    }

    Ok(())
}

async fn detect_tool_version(tool: SecurityTool) -> Option<String> {
    let mut command = version_command(tool);
    let output = command.output().await.ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !stdout.is_empty() {
        return match tool {
            SecurityTool::DockerScout => parse_docker_scout_version(&stdout)
                .or_else(|| Some(stdout.lines().next().unwrap_or_default().trim().to_string())),
            _ => Some(stdout.lines().next().unwrap_or_default().trim().to_string()),
        };
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    (!stderr.is_empty()).then_some(stderr.lines().next().unwrap_or_default().trim().to_string())
}

fn parse_docker_scout_version(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let (_, version) = line.split_once("version:")?;
        let version = version.trim();
        if version.is_empty() {
            return None;
        }

        Some(
            version
                .split_whitespace()
                .next()
                .unwrap_or(version)
                .to_string(),
        )
    })
}

fn version_command(tool: SecurityTool) -> Command {
    match tool {
        SecurityTool::Grype => {
            let mut command = Command::new("grype");
            command.arg("--version");
            command
        }
        SecurityTool::Trivy => {
            let mut command = Command::new("trivy");
            command.arg("--version");
            command
        }
        SecurityTool::DockerScout => {
            let mut command = Command::new("docker");
            command.args(["scout", "version"]);
            command
        }
    }
}

fn scan_command(tool: SecurityTool, image_reference: &str) -> Command {
    match tool {
        SecurityTool::Grype => {
            let mut command = Command::new("grype");
            command.args([image_reference, "-o", "json"]);
            command
        }
        SecurityTool::Trivy => {
            let mut command = Command::new("trivy");
            command.args(["image", "--format", "json", image_reference]);
            command
        }
        SecurityTool::DockerScout => {
            let mut command = Command::new("docker");
            command.args(["scout", "cves", image_reference, "--format", "sarif"]);
            command
        }
    }
}

async fn execute_tool_scan(tool: SecurityTool, image: &Image) -> DomainResult<Value> {
    let image_reference = image_reference(image);
    let output = scan_command(tool, &image_reference)
        .output()
        .await
        .map_err(|e| {
            DomainError::OperationFailed(format!("Failed to execute {}: {e}", tool.display_name()))
        })?;

    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(value) = serde_json::from_str::<Value>(&stdout) {
            return Ok(value);
        }
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(DomainError::OperationFailed(format!(
            "{} scan failed for {}: {}",
            tool.display_name(),
            image_reference,
            if stderr.is_empty() {
                format!("exit status {}", output.status)
            } else {
                stderr
            }
        )));
    }

    Err(DomainError::Serialization(format!(
        "{} did not return valid JSON for {}",
        tool.display_name(),
        image_reference
    )))
}

fn parse_findings(tool: SecurityTool, payload: &Value) -> Vec<SecurityFinding> {
    let findings = match tool {
        SecurityTool::Grype => parse_grype_findings(payload),
        SecurityTool::Trivy => parse_trivy_findings(payload),
        SecurityTool::DockerScout => parse_docker_scout_findings(payload),
    };

    let mut deduped = BTreeMap::new();
    for finding in findings {
        let key = format!(
            "{}|{}|{}",
            finding.vulnerability_id.trim().to_ascii_uppercase(),
            finding.package_name.trim().to_ascii_lowercase(),
            finding.installed_version.trim().to_ascii_lowercase()
        );
        match deduped.entry(key) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(finding);
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                merge_finding(entry.get_mut(), &finding);
            }
        }
    }
    let mut findings: Vec<_> = deduped.into_values().collect();
    sort_findings(&mut findings);
    findings
}

fn parse_grype_findings(payload: &Value) -> Vec<SecurityFinding> {
    payload
        .get("matches")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let vulnerability = entry.get("vulnerability")?;
            let artifact = entry.get("artifact")?;
            let vulnerability_id = string_field(vulnerability, &["id"])?;
            let package_name = string_field(artifact, &["name"])?;
            Some(SecurityFinding {
                vulnerability_id,
                package_name,
                installed_version: string_field(artifact, &["version"]).unwrap_or_default(),
                severity: VulnerabilitySeverity::from_label(
                    &string_field(vulnerability, &["severity"]).unwrap_or_default(),
                ),
                title: string_field(vulnerability, &["dataSource"]),
                description: string_field(vulnerability, &["description"]),
                fixed_version: vulnerability
                    .get("fix")
                    .and_then(|fix| fix.get("versions"))
                    .and_then(Value::as_array)
                    .and_then(|versions| versions.first())
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
                references: collect_reference_links(
                    vulnerability,
                    &[("urls", "Reference"), ("references", "Reference")],
                ),
                source_tool: SecurityTool::Grype,
            })
        })
        .collect()
}

fn parse_trivy_findings(payload: &Value) -> Vec<SecurityFinding> {
    payload
        .get("Results")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .flat_map(|result| {
            result
                .get("Vulnerabilities")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
        })
        .filter_map(|entry| {
            let vulnerability_id = string_field(&entry, &["VulnerabilityID"])?;
            let package_name = string_field(&entry, &["PkgName"])?;
            Some(SecurityFinding {
                vulnerability_id,
                package_name,
                installed_version: string_field(&entry, &["InstalledVersion"]).unwrap_or_default(),
                severity: VulnerabilitySeverity::from_label(
                    &string_field(&entry, &["Severity"]).unwrap_or_default(),
                ),
                title: string_field(&entry, &["Title"]),
                description: string_field(&entry, &["Description"]),
                fixed_version: string_field(&entry, &["FixedVersion"]),
                references: collect_reference_links(
                    &entry,
                    &[("PrimaryURL", "Primary"), ("References", "Reference")],
                ),
                source_tool: SecurityTool::Trivy,
            })
        })
        .collect()
}

fn parse_docker_scout_findings(payload: &Value) -> Vec<SecurityFinding> {
    let sarif_findings = parse_docker_scout_sarif(payload);
    if !sarif_findings.is_empty() {
        return sarif_findings;
    }

    let mut entries = Vec::new();
    collect_docker_scout_entries(payload, &mut entries);
    entries
}

fn parse_docker_scout_sarif(payload: &Value) -> Vec<SecurityFinding> {
    let mut findings = Vec::new();
    let runs = match payload.get("runs").and_then(Value::as_array) {
        Some(runs) => runs,
        None => return findings,
    };

    for run in runs {
        let rules = run
            .get("tool")
            .and_then(|tool| tool.get("driver"))
            .and_then(|driver| driver.get("rules"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let rule_map: HashMap<String, Value> = rules
            .into_iter()
            .filter_map(|rule| {
                let id = string_field(&rule, &["id"])?;
                Some((id, rule))
            })
            .collect();

        let results = run
            .get("results")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for result in results {
            if let Some(finding) = parse_docker_scout_sarif_result(&result, &rule_map) {
                findings.push(finding);
            }
        }
    }

    findings
}

fn parse_docker_scout_sarif_result(
    result: &Value,
    rule_map: &HashMap<String, Value>,
) -> Option<SecurityFinding> {
    let vulnerability_id = string_field(result, &["ruleId"])?;
    let rule = rule_map.get(&vulnerability_id);
    let properties = rule.and_then(|rule| rule.get("properties"));
    let package_purl = properties.and_then(|properties| {
        properties
            .get("purls")
            .and_then(Value::as_array)
            .and_then(|values| values.first())
            .and_then(Value::as_str)
            .map(ToString::to_string)
    });

    let (package_name, installed_version) = package_purl
        .as_deref()
        .and_then(parse_package_purl)
        .or_else(|| parse_docker_scout_message_package(result))?;

    let severity = first_non_empty(&[
        properties.and_then(|properties| string_field(properties, &["cvssV3_severity"])),
        properties.and_then(|properties| {
            properties
                .get("tags")
                .and_then(Value::as_array)
                .and_then(|tags| tags.first())
                .and_then(Value::as_str)
                .map(ToString::to_string)
        }),
        string_field(result, &["level"]),
    ])
    .map(|value| VulnerabilitySeverity::from_label(&value))
    .unwrap_or(VulnerabilitySeverity::Unknown);

    Some(SecurityFinding {
        vulnerability_id,
        package_name,
        installed_version,
        severity,
        title: rule.and_then(|rule| {
            first_non_empty(&[
                string_field(rule, &["shortDescription", "text"]),
                string_field(rule, &["name"]),
            ])
        }),
        description: first_non_empty(&[
            string_field(result, &["message", "text"]),
            rule.and_then(|rule| string_field(rule, &["help", "markdown"])),
            rule.and_then(|rule| string_field(rule, &["help", "text"])),
        ]),
        fixed_version: properties
            .and_then(|properties| string_field(properties, &["fixed_version"])),
        references: collect_reference_links_from_values(
            &[
                rule.and_then(|rule| rule.get("helpUri").cloned()),
                properties.and_then(|properties| properties.get("links").cloned()),
            ],
            "Reference",
        ),
        source_tool: SecurityTool::DockerScout,
    })
}

fn parse_docker_scout_message_package(result: &Value) -> Option<(String, String)> {
    let message = string_field(result, &["message", "text"])?;
    for line in message.lines() {
        let trimmed = line.trim();
        if let Some(purl) = trimmed.strip_prefix("Package          :") {
            if let Some(parsed) = parse_package_purl(purl.trim()) {
                return Some(parsed);
            }
        }
    }
    None
}

fn parse_package_purl(purl: &str) -> Option<(String, String)> {
    let trimmed = purl.trim();
    let without_prefix = trimmed.strip_prefix("pkg:")?;
    let name_version = without_prefix.rsplit('/').next()?;
    let name_version = name_version.split('?').next().unwrap_or(name_version);
    let (name, version) = name_version.split_once('@')?;
    Some((name.to_string(), version.to_string()))
}

fn collect_docker_scout_entries(payload: &Value, entries: &mut Vec<SecurityFinding>) {
    match payload {
        Value::Array(array) => {
            for entry in array {
                if let Some(finding) = parse_docker_scout_entry(entry) {
                    entries.push(finding);
                } else {
                    collect_docker_scout_entries(entry, entries);
                }
            }
        }
        Value::Object(map) => {
            if let Some(finding) = parse_docker_scout_entry(payload) {
                entries.push(finding);
                return;
            }
            for value in map.values() {
                collect_docker_scout_entries(value, entries);
            }
        }
        _ => {}
    }
}

fn parse_docker_scout_entry(entry: &Value) -> Option<SecurityFinding> {
    let vulnerability_id = first_non_empty(&[
        string_field(entry, &["vulnerability"]),
        string_field(entry, &["vulnerability_id"]),
        string_field(entry, &["id"]),
        string_field(entry, &["cve"]),
    ])?;

    let package_name = if let Some(package) = entry.get("package") {
        match package {
            Value::String(value) => Some(value.clone()),
            Value::Object(_) => first_non_empty(&[
                string_field(package, &["name"]),
                string_field(package, &["package"]),
            ]),
            _ => None,
        }
    } else {
        first_non_empty(&[
            string_field(entry, &["package_name"]),
            string_field(entry, &["pkg_name"]),
            string_field(entry, &["package"]),
        ])
    }?;

    Some(SecurityFinding {
        vulnerability_id,
        package_name,
        installed_version: first_non_empty(&[
            string_field(entry, &["version"]),
            string_field(entry, &["installed_version"]),
            string_field(entry, &["package_version"]),
        ])
        .unwrap_or_default(),
        severity: VulnerabilitySeverity::from_label(
            &first_non_empty(&[
                string_field(entry, &["severity"]),
                string_field(entry, &["Severity"]),
            ])
            .unwrap_or_default(),
        ),
        title: first_non_empty(&[
            string_field(entry, &["title"]),
            string_field(entry, &["name"]),
        ]),
        description: string_field(entry, &["description"]),
        fixed_version: first_non_empty(&[
            string_field(entry, &["fix_version"]),
            string_field(entry, &["fixed_version"]),
        ]),
        references: collect_reference_links(
            entry,
            &[
                ("url", "Reference"),
                ("urls", "Reference"),
                ("reference", "Reference"),
                ("references", "Reference"),
                ("advisory_url", "Advisory"),
                ("advisoryUrl", "Advisory"),
            ],
        ),
        source_tool: SecurityTool::DockerScout,
    })
}

fn collect_reference_links(value: &Value, fields: &[(&str, &str)]) -> Vec<SecurityReferenceLink> {
    let mut links = BTreeSet::new();
    for (field, default_label) in fields {
        if let Some(field_value) = value.get(*field) {
            extend_reference_links(&mut links, field_value, default_label);
        }
    }
    links.into_iter().collect()
}

fn collect_reference_links_from_values(
    values: &[Option<Value>],
    default_label: &str,
) -> Vec<SecurityReferenceLink> {
    let mut links = BTreeSet::new();
    for value in values.iter().flatten() {
        extend_reference_links(&mut links, value, default_label);
    }
    links.into_iter().collect()
}

fn extend_reference_links(
    links: &mut BTreeSet<SecurityReferenceLink>,
    value: &Value,
    default_label: &str,
) {
    match value {
        Value::String(url) => {
            if let Some(link) = make_reference_link(default_label, url) {
                links.insert(link);
            }
        }
        Value::Array(items) => {
            for item in items {
                extend_reference_links(links, item, default_label);
            }
        }
        Value::Object(map) => {
            let label = first_non_empty(&[
                map.get("title")
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
                map.get("label")
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
                map.get("source")
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
                Some(default_label.to_string()),
            ])
            .unwrap_or_else(|| default_label.to_string());

            for url_field in ["url", "href", "URI", "uri"] {
                if let Some(url) = map.get(url_field).and_then(Value::as_str) {
                    if let Some(link) = make_reference_link(&label, url) {
                        links.insert(link);
                    }
                }
            }
        }
        _ => {}
    }
}

fn make_reference_link(label: &str, url: &str) -> Option<SecurityReferenceLink> {
    let normalized_url = normalize_reference_url(url)?;
    Some(SecurityReferenceLink {
        label: if label.trim().is_empty() {
            "Reference".into()
        } else {
            label.trim().to_string()
        },
        url: normalized_url,
    })
}

fn normalize_reference_url(url: &str) -> Option<String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn sort_findings(findings: &mut [SecurityFinding]) {
    findings.sort_by(|left, right| {
        right
            .severity
            .rank()
            .cmp(&left.severity.rank())
            .then_with(|| left.vulnerability_id.cmp(&right.vulnerability_id))
            .then_with(|| left.package_name.cmp(&right.package_name))
            .then_with(|| left.installed_version.cmp(&right.installed_version))
    });
}

fn merge_finding(target: &mut SecurityFinding, incoming: &SecurityFinding) {
    if incoming.severity.rank() > target.severity.rank() {
        target.severity = incoming.severity;
    }

    merge_optional_string(&mut target.title, &incoming.title);
    merge_optional_string(&mut target.description, &incoming.description);
    merge_optional_string(&mut target.fixed_version, &incoming.fixed_version);

    let mut references: BTreeSet<SecurityReferenceLink> =
        target.references.iter().cloned().collect();
    references.extend(incoming.references.iter().cloned());
    target.references = references.into_iter().collect();
}

fn merge_optional_string(target: &mut Option<String>, incoming: &Option<String>) {
    match (target.as_ref(), incoming.as_ref()) {
        (None, Some(value)) if !value.trim().is_empty() => {
            *target = Some(value.clone());
        }
        (Some(current), Some(candidate)) if candidate.trim().len() > current.trim().len() => {
            *target = Some(candidate.clone());
        }
        _ => {}
    }
}

fn build_image_tool_states(
    image: &Image,
    tool_statuses: &HashMap<SecurityTool, SecurityToolStatus>,
    stored_reports: &[StoredToolReport],
    runtime: &HashMap<(String, SecurityTool), RuntimeToolState>,
    selected_tools: &BTreeSet<SecurityTool>,
    now: DateTime<Utc>,
) -> Vec<SecurityImageToolStatus> {
    SecurityTool::all()
        .iter()
        .map(|tool| {
            let report = stored_reports.iter().find(|report| report.tool == *tool);
            let runtime_state = runtime.get(&(image.id.clone(), *tool));
            let is_selected = selected_tools.contains(tool);
            let report_is_fresh = report.is_some_and(|report| is_report_fresh(report, now));
            let tool_status =
                tool_statuses
                    .get(tool)
                    .cloned()
                    .unwrap_or_else(|| SecurityToolStatus {
                        tool: *tool,
                        available: false,
                        selected: false,
                        version: None,
                        install_hint: install_hint_for(*tool),
                    });

            SecurityImageToolStatus {
                tool: *tool,
                state: runtime_state.map(|state| state.state).unwrap_or_else(|| {
                    if report_is_fresh {
                        SecurityScanState::Completed
                    } else {
                        SecurityScanState::Idle
                    }
                }),
                available: tool_status.available,
                selected: tool_status.selected,
                findings_count: runtime_state
                    .and_then(|state| state.findings_count)
                    .or_else(|| {
                        if report_is_fresh {
                            report.map(|report| report.findings.len() as u32)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0),
                last_scanned_at: runtime_state
                    .map(|state| state.updated_at.clone())
                    .or_else(|| report.map(|report| report.generated_at.clone())),
                message: runtime_state
                    .and_then(|state| state.message.clone())
                    .or_else(|| {
                        if is_selected && report.is_some() && !report_is_fresh {
                            Some(format!(
                                "Stored results are older than {REPORT_MAX_AGE_DAYS} days and will be refreshed."
                            ))
                        } else if report_is_fresh {
                            report.map(|report| {
                                format!(
                                    "{} findings from {}",
                                    report.findings.len(),
                                    tool.display_name()
                                )
                            })
                        } else {
                            None
                        }
                    }),
            }
        })
        .collect()
}

async fn update_runtime_state(
    runtime_states: &Arc<RwLock<HashMap<(String, SecurityTool), RuntimeToolState>>>,
    image_id: &str,
    tool: SecurityTool,
    state: RuntimeToolState,
) {
    runtime_states
        .write()
        .await
        .insert((image_id.to_string(), tool), state);
}

async fn write_report(results_dir: &Path, report: &StoredToolReport) -> DomainResult<()> {
    let image_dir = results_dir.join(sanitize_path_component(&report.image_id));
    fs::create_dir_all(&image_dir)
        .await
        .map_err(|e| DomainError::Config(format!("Cannot create image report dir: {e}")))?;

    let path = report_file_path(results_dir, &report.image_id, report.tool);
    let payload = serde_json::to_vec_pretty(report)
        .map_err(|e| DomainError::Serialization(format!("Cannot serialize report: {e}")))?;
    fs::write(&path, payload)
        .await
        .map_err(|e| DomainError::Config(format!("Cannot persist report file: {e}")))?;
    Ok(())
}

async fn delete_report_file(
    results_dir: &Path,
    image_id: &str,
    tool: SecurityTool,
) -> DomainResult<()> {
    let path = report_file_path(results_dir, image_id, tool);
    match fs::remove_file(&path).await {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(DomainError::Config(format!(
                "Cannot delete stale report file {}: {error}",
                path.display()
            )))
        }
    }

    let image_dir = results_dir.join(sanitize_path_component(image_id));
    match fs::remove_dir(&image_dir).await {
        Ok(()) => {}
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
            ) => {}
        Err(error) => {
            return Err(DomainError::Config(format!(
                "Cannot remove empty image report dir {}: {error}",
                image_dir.display()
            )))
        }
    }

    Ok(())
}

async fn read_reports_for_image(
    results_dir: &Path,
    image_id: &str,
) -> DomainResult<Vec<StoredToolReport>> {
    let image_dir = results_dir.join(sanitize_path_component(image_id));
    let mut reports = Vec::new();
    let mut entries = match fs::read_dir(&image_dir).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(reports),
        Err(error) => {
            return Err(DomainError::Config(format!(
                "Cannot read security results for {image_id}: {error}"
            )))
        }
    };

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| DomainError::Config(format!("Cannot enumerate security results: {e}")))?
    {
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }

        let content = fs::read_to_string(&path).await.map_err(|e| {
            DomainError::Config(format!("Cannot read report file {}: {e}", path.display()))
        })?;
        let report: StoredToolReport = serde_json::from_str(&content).map_err(|e| {
            DomainError::Serialization(format!("Cannot parse report file {}: {e}", path.display()))
        })?;
        reports.push(report);
    }

    reports.sort_by_key(|report| report.tool);
    Ok(reports)
}

async fn build_scan_queue(
    images: &[Image],
    results_dir: &Path,
    tool: SecurityTool,
    now: DateTime<Utc>,
) -> DomainResult<Vec<Image>> {
    let mut missing = Vec::new();
    let mut stale = Vec::new();

    for image in images {
        let stored_reports = read_reports_for_image(results_dir, &image.id).await?;
        let tool_report = stored_reports.iter().find(|report| report.tool == tool);
        match scan_priority_for_tool(tool_report, now) {
            Some(ScanPriority::Missing) => missing.push(image.clone()),
            Some(ScanPriority::Stale(generated_at)) => {
                delete_report_file(results_dir, &image.id, tool).await?;
                stale.push((generated_at, image.clone()));
            }
            None => {}
        }
    }

    stale.sort_by_key(|(generated_at, _)| *generated_at);
    missing.extend(stale.into_iter().map(|(_, image)| image));
    Ok(missing)
}

fn dedup_findings(
    image_id: &str,
    reports: &[StoredToolReport],
) -> BTreeMap<String, SecurityFinding> {
    let mut dedup = BTreeMap::new();
    for report in reports {
        for finding in &report.findings {
            match dedup.entry(finding.dedup_key(image_id)) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(finding.clone());
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
                    merge_finding(entry.get_mut(), finding);
                }
            }
        }
    }
    dedup
}

fn count_findings(findings: Vec<SecurityFinding>) -> Vec<SeverityCount> {
    let mut counts = zero_counts();
    for finding in findings {
        if let Some(bucket) = counts
            .iter_mut()
            .find(|bucket| bucket.severity == finding.severity)
        {
            bucket.count += 1;
        }
    }
    counts
}

fn zero_counts() -> Vec<SeverityCount> {
    VulnerabilitySeverity::all()
        .iter()
        .map(|severity| SeverityCount {
            severity: *severity,
            count: 0,
        })
        .collect()
}

fn add_counts(target: &mut [SeverityCount], source: &[SeverityCount]) {
    for bucket in source {
        if let Some(target_bucket) = target
            .iter_mut()
            .find(|target_bucket| target_bucket.severity == bucket.severity)
        {
            target_bucket.count += bucket.count;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScanPriority {
    Missing,
    Stale(DateTime<Utc>),
}

fn scan_priority_for_tool(
    report: Option<&StoredToolReport>,
    now: DateTime<Utc>,
) -> Option<ScanPriority> {
    match report {
        None => Some(ScanPriority::Missing),
        Some(report) if is_report_fresh(report, now) => None,
        Some(report) => Some(ScanPriority::Stale(
            report_generated_at(report).unwrap_or(now),
        )),
    }
}

fn filter_effective_reports(
    reports: &[StoredToolReport],
    selected_tools: &BTreeSet<SecurityTool>,
    now: DateTime<Utc>,
) -> Vec<StoredToolReport> {
    reports
        .iter()
        .filter(|report| selected_tools.contains(&report.tool) && is_report_fresh(report, now))
        .cloned()
        .collect()
}

fn filter_fresh_reports(reports: &[StoredToolReport], now: DateTime<Utc>) -> Vec<StoredToolReport> {
    reports
        .iter()
        .filter(|report| is_report_fresh(report, now))
        .cloned()
        .collect()
}

fn string_field(value: &Value, path: &[&str]) -> Option<String> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }

    current.as_str().map(ToString::to_string)
}

fn first_non_empty(candidates: &[Option<String>]) -> Option<String> {
    candidates
        .iter()
        .flatten()
        .find(|value| !value.trim().is_empty())
        .cloned()
}

fn image_reference(image: &Image) -> String {
    if !image.repo_name.is_empty() && !image.tag.is_empty() {
        image.full_name()
    } else if !image.repo_name.is_empty() {
        image.repo_name.clone()
    } else {
        image.id.clone()
    }
}

fn sanitize_path_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn tool_file_stem(tool: SecurityTool) -> &'static str {
    match tool {
        SecurityTool::Grype => "grype",
        SecurityTool::Trivy => "trivy",
        SecurityTool::DockerScout => "docker_scout",
    }
}

fn report_file_path(results_dir: &Path, image_id: &str, tool: SecurityTool) -> PathBuf {
    results_dir
        .join(sanitize_path_component(image_id))
        .join(format!("{}.json", tool_file_stem(tool)))
}

fn now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

fn report_generated_at(report: &StoredToolReport) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&report.generated_at)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

fn is_report_fresh(report: &StoredToolReport, now: DateTime<Utc>) -> bool {
    report_generated_at(report)
        .map(|generated_at| {
            now.signed_duration_since(generated_at) <= Duration::days(REPORT_MAX_AGE_DAYS)
        })
        .unwrap_or(false)
}

fn install_hint_for(tool: SecurityTool) -> SecurityInstallHint {
    match tool {
        SecurityTool::Grype => SecurityInstallHint {
            title: format!("Install {}", tool.display_name()),
            description: "Install Grype to enable image vulnerability scanning.".into(),
            commands: grype_install_commands(),
            note: Some("After installation, reopen the app or refresh the security screen.".into()),
        },
        SecurityTool::Trivy => SecurityInstallHint {
            title: format!("Install {}", tool.display_name()),
            description: "Install Trivy to enable image vulnerability scanning.".into(),
            commands: trivy_install_commands(),
            note: Some("After installation, reopen the app or refresh the security screen.".into()),
        },
        SecurityTool::DockerScout => SecurityInstallHint {
            title: format!("Install {}", tool.display_name()),
            description: "Install Docker Scout or enable the Docker CLI plugin.".into(),
            commands: docker_scout_install_commands(),
            note: Some("Container Desktop uses `docker scout`, so Docker CLI access must work from this machine.".into()),
        },
    }
}

#[cfg(target_os = "linux")]
fn grype_install_commands() -> Vec<String> {
    vec![
        "Homebrew: brew tap anchore/grype && brew install grype".into(),
        "Install script: curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh | sh -s -- -b /usr/local/bin".into(),
    ]
}

#[cfg(target_os = "macos")]
fn grype_install_commands() -> Vec<String> {
    vec![
        "Homebrew: brew tap anchore/grype && brew install grype".into(),
        "Install script: curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh | sh -s -- -b /usr/local/bin".into(),
    ]
}

#[cfg(target_os = "windows")]
fn grype_install_commands() -> Vec<String> {
    vec![
        "Scoop: scoop bucket add grype https://github.com/anchore/scoop-grype".into(),
        "Scoop: scoop install grype".into(),
        "Manual: download the latest Grype release and add grype.exe to PATH".into(),
    ]
}

#[cfg(target_os = "linux")]
fn trivy_install_commands() -> Vec<String> {
    vec![
        "Install script: curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sudo sh -s -- -b /usr/local/bin".into(),
        "Alternative package install: download the latest Trivy .deb/.rpm from GitHub releases".into(),
    ]
}

#[cfg(target_os = "macos")]
fn trivy_install_commands() -> Vec<String> {
    vec![
        "Homebrew: brew install aquasecurity/trivy/trivy".into(),
        "Install script: curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin".into(),
    ]
}

#[cfg(target_os = "windows")]
fn trivy_install_commands() -> Vec<String> {
    vec![
        "Chocolatey: choco install trivy".into(),
        "Manual: download the latest Windows Trivy zip and add trivy.exe to PATH".into(),
    ]
}

#[cfg(target_os = "linux")]
fn docker_scout_install_commands() -> Vec<String> {
    vec![
        "Docker CLI plugin: docker extension install docker/scout-cli".into(),
        "Manual: download the latest Docker Scout CLI release and place the binary in PATH".into(),
    ]
}

#[cfg(target_os = "macos")]
fn docker_scout_install_commands() -> Vec<String> {
    vec![
        "Docker CLI plugin: docker extension install docker/scout-cli".into(),
        "Manual: download the latest Docker Scout CLI release and place the binary in PATH".into(),
    ]
}

#[cfg(target_os = "windows")]
fn docker_scout_install_commands() -> Vec<String> {
    vec![
        "Docker Desktop: update to a recent Docker Desktop release and open a new terminal".into(),
        "Manual: download the Docker Scout CLI Windows archive and add scout.exe to PATH".into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_image() -> Image {
        Image {
            id: "sha256:test-image".into(),
            repo_name: "nginx".into(),
            tag: "latest".into(),
            size: "100 MB".into(),
            created: "2026-01-01 00:00:00".into(),
            labels: vec![],
        }
    }

    #[test]
    fn parse_grype_payload() {
        let payload = serde_json::json!({
            "matches": [
                {
                    "artifact": { "name": "openssl", "version": "1.1.1" },
                    "vulnerability": {
                        "id": "CVE-2024-0001",
                        "severity": "High",
                        "description": "openssl issue",
                        "urls": ["https://example.com/grype/CVE-2024-0001"],
                        "fix": { "versions": ["1.1.2"] }
                    }
                }
            ]
        });

        let findings = parse_grype_findings(&payload);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].package_name, "openssl");
        assert_eq!(findings[0].severity, VulnerabilitySeverity::High);
        assert_eq!(findings[0].fixed_version.as_deref(), Some("1.1.2"));
        assert_eq!(findings[0].references.len(), 1);
    }

    #[test]
    fn parse_trivy_payload() {
        let payload = serde_json::json!({
            "Results": [
                {
                    "Vulnerabilities": [
                        {
                            "VulnerabilityID": "CVE-2024-0002",
                            "PkgName": "glibc",
                            "InstalledVersion": "2.31",
                            "Severity": "CRITICAL",
                            "Title": "glibc issue",
                            "Description": "details",
                            "FixedVersion": "2.32",
                            "PrimaryURL": "https://example.com/trivy/CVE-2024-0002",
                            "References": ["https://example.com/trivy/extra"]
                        }
                    ]
                }
            ]
        });

        let findings = parse_trivy_findings(&payload);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].package_name, "glibc");
        assert_eq!(findings[0].severity, VulnerabilitySeverity::Critical);
        assert_eq!(findings[0].references.len(), 2);
    }

    #[test]
    fn parse_docker_scout_payload() {
        let payload = serde_json::json!([
            {
                "vulnerability": "CVE-2024-0003",
                "package": "zlib",
                "version": "1.2.3",
                "severity": "medium",
                "fix_version": "1.2.4",
                "advisory_url": "https://example.com/scout/CVE-2024-0003"
            }
        ]);

        let findings = parse_docker_scout_findings(&payload);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].package_name, "zlib");
        assert_eq!(findings[0].severity, VulnerabilitySeverity::Medium);
        assert_eq!(findings[0].references.len(), 1);
    }

    #[test]
    fn parse_docker_scout_sarif_payload() {
        let payload = serde_json::json!({
            "version": "2.1.0",
            "runs": [
                {
                    "tool": {
                        "driver": {
                            "rules": [
                                {
                                    "id": "CVE-2025-60876",
                                    "name": "OsPackageVulnerability",
                                    "shortDescription": { "text": "CVE-2025-60876" },
                                    "helpUri": "https://scout.docker.com/v/CVE-2025-60876",
                                    "properties": {
                                        "cvssV3_severity": "MEDIUM",
                                        "fixed_version": "not fixed",
                                        "purls": [
                                            "pkg:apk/alpine/busybox@1.37.0-r30?os_name=alpine&os_version=3.23"
                                        ]
                                    }
                                }
                            ]
                        }
                    },
                    "results": [
                        {
                            "ruleId": "CVE-2025-60876",
                            "message": {
                                "text": "Package          :pkg:apk/alpine/busybox@1.37.0-r30?os_name=alpine&os_version=3.23"
                            }
                        }
                    ]
                }
            ]
        });

        let findings = parse_docker_scout_findings(&payload);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].vulnerability_id, "CVE-2025-60876");
        assert_eq!(findings[0].package_name, "busybox");
        assert_eq!(findings[0].installed_version, "1.37.0-r30");
        assert_eq!(findings[0].severity, VulnerabilitySeverity::Medium);
        assert_eq!(findings[0].references.len(), 1);
    }

    #[test]
    fn parse_docker_scout_version_from_version_line() {
        let output = r#"
version: v1.20.4 (go1.25.8 - linux/amd64)
git commit: fb59552651671b31cca99eba9895522871678c46
"#;

        assert_eq!(
            parse_docker_scout_version(output).as_deref(),
            Some("v1.20.4")
        );
    }

    #[test]
    fn parse_docker_scout_version_returns_none_without_version_line() {
        let output = "git commit: fb59552651671b31cca99eba9895522871678c46";

        assert_eq!(parse_docker_scout_version(output), None);
    }

    #[test]
    fn dedup_findings_merges_same_vulnerability_across_tools() {
        let image = test_image();
        let finding = SecurityFinding {
            vulnerability_id: "CVE-2024-0004".into(),
            package_name: "curl".into(),
            installed_version: "8.0".into(),
            severity: VulnerabilitySeverity::High,
            title: None,
            description: None,
            fixed_version: None,
            references: vec![SecurityReferenceLink {
                label: "Reference".into(),
                url: "https://example.com/grype/CVE-2024-0004".into(),
            }],
            source_tool: SecurityTool::Grype,
        };

        let reports = vec![
            StoredToolReport {
                image_id: image.id.clone(),
                image_name: image_reference(&image),
                tool: SecurityTool::Grype,
                tool_version: Some("1".into()),
                generated_at: now_rfc3339(),
                findings: vec![finding.clone()],
                severity_counts: zero_counts(),
                raw_json: serde_json::json!({}),
            },
            StoredToolReport {
                image_id: image.id.clone(),
                image_name: image_reference(&image),
                tool: SecurityTool::Trivy,
                tool_version: Some("1".into()),
                generated_at: now_rfc3339(),
                findings: vec![SecurityFinding {
                    severity: VulnerabilitySeverity::Critical,
                    references: vec![SecurityReferenceLink {
                        label: "Reference".into(),
                        url: "https://example.com/trivy/CVE-2024-0004".into(),
                    }],
                    source_tool: SecurityTool::Trivy,
                    ..finding
                }],
                severity_counts: zero_counts(),
                raw_json: serde_json::json!({}),
            },
        ];

        let merged = dedup_findings(&image.id, &reports);
        assert_eq!(merged.len(), 1);
        let finding = merged.values().next().unwrap();
        assert_eq!(finding.severity, VulnerabilitySeverity::Critical);
        assert_eq!(finding.references.len(), 2);
    }

    #[test]
    fn stale_reports_require_rescan() {
        let report = StoredToolReport {
            image_id: "sha256:test-image".into(),
            image_name: "nginx:latest".into(),
            tool: SecurityTool::Grype,
            tool_version: Some("1".into()),
            generated_at: (Utc::now() - Duration::days(REPORT_MAX_AGE_DAYS + 1)).to_rfc3339(),
            findings: vec![],
            severity_counts: zero_counts(),
            raw_json: serde_json::json!({}),
        };

        assert_eq!(
            scan_priority_for_tool(Some(&report), Utc::now()),
            Some(ScanPriority::Stale(report_generated_at(&report).unwrap()))
        );
    }

    #[test]
    fn fresh_selected_reports_filter_out_stale_and_unselected() {
        let fresh_grype = StoredToolReport {
            image_id: "sha256:test-image".into(),
            image_name: "nginx:latest".into(),
            tool: SecurityTool::Grype,
            tool_version: Some("1".into()),
            generated_at: Utc::now().to_rfc3339(),
            findings: vec![],
            severity_counts: zero_counts(),
            raw_json: serde_json::json!({}),
        };
        let stale_trivy = StoredToolReport {
            image_id: "sha256:test-image".into(),
            image_name: "nginx:latest".into(),
            tool: SecurityTool::Trivy,
            tool_version: Some("1".into()),
            generated_at: (Utc::now() - Duration::days(REPORT_MAX_AGE_DAYS + 1)).to_rfc3339(),
            findings: vec![],
            severity_counts: zero_counts(),
            raw_json: serde_json::json!({}),
        };
        let selected_tools = BTreeSet::from([SecurityTool::Grype]);

        let reports = filter_effective_reports(
            &[fresh_grype.clone(), stale_trivy],
            &selected_tools,
            Utc::now(),
        );

        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].tool, SecurityTool::Grype);
    }

    #[test]
    fn stored_report_deserializes_when_legacy_findings_lack_references() {
        let content = serde_json::json!({
            "image_id": "sha256:test-image",
            "image_name": "nginx:latest",
            "tool": "Grype",
            "tool_version": "1.0.0",
            "generated_at": "2026-05-13T10:00:00Z",
            "findings": [
                {
                    "vulnerability_id": "CVE-2024-0005",
                    "package_name": "busybox",
                    "installed_version": "1.36",
                    "severity": "Low",
                    "title": null,
                    "description": null,
                    "fixed_version": null,
                    "source_tool": "Grype"
                }
            ],
            "severity_counts": [
                { "severity": "Critical", "count": 0 },
                { "severity": "High", "count": 0 },
                { "severity": "Medium", "count": 0 },
                { "severity": "Low", "count": 1 },
                { "severity": "Negligible", "count": 0 },
                { "severity": "Unknown", "count": 0 }
            ],
            "raw_json": {}
        })
        .to_string();

        let report: StoredToolReport = serde_json::from_str(&content).unwrap();
        assert_eq!(report.findings.len(), 1);
        assert!(report.findings[0].references.is_empty());
    }

    #[test]
    fn parse_findings_sorts_by_severity_descending() {
        let payload = serde_json::json!({
            "Results": [
                {
                    "Vulnerabilities": [
                        {
                            "VulnerabilityID": "CVE-2024-0100",
                            "PkgName": "pkg-low",
                            "InstalledVersion": "1",
                            "Severity": "LOW"
                        },
                        {
                            "VulnerabilityID": "CVE-2024-0101",
                            "PkgName": "pkg-critical",
                            "InstalledVersion": "1",
                            "Severity": "CRITICAL"
                        }
                    ]
                }
            ]
        });

        let findings = parse_findings(SecurityTool::Trivy, &payload);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].severity, VulnerabilitySeverity::Critical);
        assert_eq!(findings[1].severity, VulnerabilitySeverity::Low);
    }

    #[tokio::test]
    async fn report_storage_roundtrip() {
        let temp_dir = std::env::temp_dir().join(format!(
            "container_desktop_security_test_{}",
            now_rfc3339().replace(':', "_")
        ));
        let results_dir = temp_dir.join("results");
        let image = test_image();
        let report = StoredToolReport {
            image_id: image.id.clone(),
            image_name: image_reference(&image),
            tool: SecurityTool::Grype,
            tool_version: Some("1.0.0".into()),
            generated_at: now_rfc3339(),
            findings: vec![SecurityFinding {
                vulnerability_id: "CVE-2024-0005".into(),
                package_name: "busybox".into(),
                installed_version: "1.36".into(),
                severity: VulnerabilitySeverity::Low,
                title: None,
                description: None,
                fixed_version: None,
                references: vec![],
                source_tool: SecurityTool::Grype,
            }],
            severity_counts: vec![SeverityCount {
                severity: VulnerabilitySeverity::Low,
                count: 1,
            }],
            raw_json: serde_json::json!({ "matches": [] }),
        };

        write_report(&results_dir, &report).await.unwrap();
        let loaded = read_reports_for_image(&results_dir, &image.id)
            .await
            .unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].tool, SecurityTool::Grype);
        assert_eq!(loaded[0].findings[0].package_name, "busybox");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn stale_report_file_is_deleted_before_requeue() {
        let temp_dir = std::env::temp_dir().join(format!(
            "container_desktop_security_stale_cleanup_test_{}",
            now_rfc3339().replace(':', "_")
        ));
        let results_dir = temp_dir.join("results");
        let image = test_image();
        let report = StoredToolReport {
            image_id: image.id.clone(),
            image_name: image_reference(&image),
            tool: SecurityTool::Grype,
            tool_version: Some("1.0.0".into()),
            generated_at: (Utc::now() - Duration::days(REPORT_MAX_AGE_DAYS + 1)).to_rfc3339(),
            findings: vec![],
            severity_counts: zero_counts(),
            raw_json: serde_json::json!({}),
        };

        write_report(&results_dir, &report).await.unwrap();
        let report_path = report_file_path(&results_dir, &image.id, SecurityTool::Grype);
        assert!(report_path.exists());

        let queue = build_scan_queue(
            &[image.clone()],
            &results_dir,
            SecurityTool::Grype,
            Utc::now(),
        )
        .await
        .unwrap();

        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].id, image.id);
        assert!(!report_path.exists());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
