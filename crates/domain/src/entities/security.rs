use serde::{Deserialize, Serialize};

/// Supported vulnerability scanner tools.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SecurityTool {
    Grype,
    Trivy,
    DockerScout,
}

impl SecurityTool {
    /// Returns all supported tools in UI order.
    pub fn all() -> &'static [SecurityTool] {
        &[
            SecurityTool::Grype,
            SecurityTool::Trivy,
            SecurityTool::DockerScout,
        ]
    }

    /// Returns a user-facing display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            SecurityTool::Grype => "Grype",
            SecurityTool::Trivy => "Trivy",
            SecurityTool::DockerScout => "Docker Scout",
        }
    }
}

/// Persisted security preferences.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SecuritySettings {
    /// Enabled scanners selected by the user.
    #[serde(default)]
    pub selected_tools: Vec<SecurityTool>,
}

/// Canonical vulnerability severity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
    Negligible,
    Unknown,
}

impl VulnerabilitySeverity {
    /// Returns severities in descending importance order.
    pub fn all() -> &'static [VulnerabilitySeverity] {
        &[
            VulnerabilitySeverity::Critical,
            VulnerabilitySeverity::High,
            VulnerabilitySeverity::Medium,
            VulnerabilitySeverity::Low,
            VulnerabilitySeverity::Negligible,
            VulnerabilitySeverity::Unknown,
        ]
    }

    /// Parses a tool-provided severity string.
    pub fn from_label(label: &str) -> Self {
        match label.trim().to_ascii_lowercase().as_str() {
            "critical" => VulnerabilitySeverity::Critical,
            "high" => VulnerabilitySeverity::High,
            "medium" | "moderate" => VulnerabilitySeverity::Medium,
            "low" => VulnerabilitySeverity::Low,
            "negligible" | "info" | "informational" => VulnerabilitySeverity::Negligible,
            _ => VulnerabilitySeverity::Unknown,
        }
    }

    /// Returns a short label for display.
    pub fn label(&self) -> &'static str {
        match self {
            VulnerabilitySeverity::Critical => "Critical",
            VulnerabilitySeverity::High => "High",
            VulnerabilitySeverity::Medium => "Medium",
            VulnerabilitySeverity::Low => "Low",
            VulnerabilitySeverity::Negligible => "Negligible",
            VulnerabilitySeverity::Unknown => "Unknown",
        }
    }

    /// Severity ordering weight where higher means more critical.
    pub fn rank(&self) -> u8 {
        match self {
            VulnerabilitySeverity::Critical => 6,
            VulnerabilitySeverity::High => 5,
            VulnerabilitySeverity::Medium => 4,
            VulnerabilitySeverity::Low => 3,
            VulnerabilitySeverity::Negligible => 2,
            VulnerabilitySeverity::Unknown => 1,
        }
    }
}

/// Count of findings for one severity bucket.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeverityCount {
    pub severity: VulnerabilitySeverity,
    pub count: u32,
}

/// Reference link associated with a vulnerability finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityReferenceLink {
    pub label: String,
    pub url: String,
}

/// Installation guidance for one scanner on the current OS.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityInstallHint {
    pub title: String,
    pub description: String,
    pub commands: Vec<String>,
    pub note: Option<String>,
}

/// Runtime or persisted state of one scan execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityScanState {
    Idle,
    Running,
    Completed,
    Failed,
}

/// Availability and selection state of one scanner tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityToolStatus {
    pub tool: SecurityTool,
    pub available: bool,
    pub selected: bool,
    pub version: Option<String>,
    pub install_hint: SecurityInstallHint,
}

/// Normalized vulnerability finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityFinding {
    pub vulnerability_id: String,
    pub package_name: String,
    pub installed_version: String,
    pub severity: VulnerabilitySeverity,
    pub title: Option<String>,
    pub description: Option<String>,
    pub fixed_version: Option<String>,
    #[serde(default)]
    pub references: Vec<SecurityReferenceLink>,
    pub source_tool: SecurityTool,
}

impl SecurityFinding {
    /// Canonical de-duplication key used across tools for one image.
    pub fn dedup_key(&self, image_id: &str) -> String {
        format!(
            "{}|{}|{}|{}",
            image_id,
            canonical_fragment(&self.vulnerability_id, true),
            canonical_fragment(&self.package_name, false),
            canonical_fragment(&self.installed_version, false)
        )
    }
}

fn canonical_fragment(value: &str, uppercase: bool) -> String {
    let trimmed = value.trim();
    if uppercase {
        trimmed.to_ascii_uppercase()
    } else {
        trimmed.to_ascii_lowercase()
    }
}

/// Per-image, per-tool status shown in the overview.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityImageToolStatus {
    pub tool: SecurityTool,
    pub state: SecurityScanState,
    pub available: bool,
    pub selected: bool,
    pub findings_count: u32,
    pub last_scanned_at: Option<String>,
    pub message: Option<String>,
}

/// Consolidated security summary for one image.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityImageSummary {
    pub image_id: String,
    pub image_name: String,
    pub repo_name: String,
    pub tag: String,
    pub total_findings: u32,
    pub severity_counts: Vec<SeverityCount>,
    pub tool_statuses: Vec<SecurityImageToolStatus>,
    pub last_scanned_at: Option<String>,
}

/// Global overview rendered in the security screen.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityOverview {
    pub tools: Vec<SecurityToolStatus>,
    pub total_images: u32,
    pub scanned_images: u32,
    pub images_with_findings: u32,
    pub findings_by_severity: Vec<SeverityCount>,
    pub images: Vec<SecurityImageSummary>,
}

/// Stored report for one tool and one image.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityToolReport {
    pub tool: SecurityTool,
    pub state: SecurityScanState,
    pub tool_version: Option<String>,
    pub generated_at: Option<String>,
    pub findings: Vec<SecurityFinding>,
    pub severity_counts: Vec<SeverityCount>,
    pub message: Option<String>,
}

/// Detailed report for one image across all available tools.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageSecurityReport {
    pub image_id: String,
    pub image_name: String,
    pub reports: Vec<SecurityToolReport>,
}

/// Background progress update emitted during scans.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityScanProgress {
    pub tool: SecurityTool,
    pub image_id: String,
    pub image_name: String,
    pub state: SecurityScanState,
    pub findings_count: Option<u32>,
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_display_names_are_stable() {
        assert_eq!(SecurityTool::Grype.display_name(), "Grype");
        assert_eq!(SecurityTool::Trivy.display_name(), "Trivy");
        assert_eq!(SecurityTool::DockerScout.display_name(), "Docker Scout");
        assert_eq!(SecurityTool::all().len(), 3);
    }

    #[test]
    fn severity_parsing_normalizes_aliases() {
        assert_eq!(
            VulnerabilitySeverity::from_label("critical"),
            VulnerabilitySeverity::Critical
        );
        assert_eq!(
            VulnerabilitySeverity::from_label("Moderate"),
            VulnerabilitySeverity::Medium
        );
        assert_eq!(
            VulnerabilitySeverity::from_label("informational"),
            VulnerabilitySeverity::Negligible
        );
        assert_eq!(
            VulnerabilitySeverity::from_label("weird"),
            VulnerabilitySeverity::Unknown
        );
    }

    #[test]
    fn finding_dedup_key_uses_image_and_package_identity() {
        let finding = SecurityFinding {
            vulnerability_id: "CVE-123".into(),
            package_name: "openssl".into(),
            installed_version: "1.0.0".into(),
            severity: VulnerabilitySeverity::High,
            title: None,
            description: None,
            fixed_version: None,
            references: vec![],
            source_tool: SecurityTool::Grype,
        };

        assert_eq!(
            finding.dedup_key("sha256:abc"),
            "sha256:abc|CVE-123|openssl|1.0.0"
        );
    }

    #[test]
    fn dedup_key_normalizes_case_and_spacing() {
        let finding = SecurityFinding {
            vulnerability_id: " cve-123 ".into(),
            package_name: " OpenSSL ".into(),
            installed_version: " 1.0.0 ".into(),
            severity: VulnerabilitySeverity::High,
            title: None,
            description: None,
            fixed_version: None,
            references: vec![],
            source_tool: SecurityTool::Grype,
        };

        assert_eq!(
            finding.dedup_key("sha256:abc"),
            "sha256:abc|CVE-123|openssl|1.0.0"
        );
    }
}
