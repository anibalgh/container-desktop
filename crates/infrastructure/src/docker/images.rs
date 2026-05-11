use super::DockerClient;
use async_trait::async_trait;
use bollard::query_parameters::ListImagesOptionsBuilder;
use domain::entities::Image;
use domain::repository::ImageRepository;
use domain::{DomainError, DomainResult};
use futures::Stream;
use futures::StreamExt;

#[async_trait]
impl ImageRepository for DockerClient {
    async fn list_images(&self) -> DomainResult<Vec<Image>> {
        let docker = self.get_docker().await?;

        let options = ListImagesOptionsBuilder::default().all(true).build();

        let images = docker
            .list_images(Some(options))
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot list images: {e}")))?;

        let result = images
            .into_iter()
            .map(|img| {
                let (repo_name, tag) = if let Some(first_tag) = img.repo_tags.first() {
                    let parts: Vec<&str> = first_tag.splitn(2, ':').collect();
                    (
                        parts.first().unwrap_or(&"").to_string(),
                        parts.get(1).unwrap_or(&"latest").to_string(),
                    )
                } else {
                    (String::new(), String::new())
                };

                Image {
                    id: img.id,
                    repo_name,
                    tag,
                    size: format_bytes(img.size),
                    created: format_timestamp(img.created),
                    labels: img
                        .labels
                        .into_iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .collect(),
                }
            })
            .collect();

        Ok(result)
    }

    async fn pull_image(
        &self,
        name: &str,
        tag: Option<&str>,
    ) -> DomainResult<Box<dyn Stream<Item = DomainResult<String>> + Unpin + Send>> {
        let docker = self.get_docker().await?;
        let full_name = match tag {
            Some(t) => format!("{name}:{t}"),
            None => name.to_string(),
        };

        let options = bollard::query_parameters::CreateImageOptionsBuilder::default()
            .from_image(&full_name)
            .build();

        let stream = docker.create_image(
            Some(options),
            None,
            Some(bollard::auth::DockerCredentials::default()),
        );

        let mapped = stream.map(|result| {
            result
                .map_err(|e| DomainError::DockerApi(format!("Pull failed: {e}")))
                .and_then(|output| {
                    if let Some(error) = output.error_detail {
                        Err(DomainError::DockerApi(format!("{error:?}")))
                    } else if let Some(status) = output.status {
                        let mut msg = status.clone();
                        if let Some(progress) = output.progress_detail {
                            if let Some(pd) = progress.current {
                                msg.push_str(&format!(" - {pd}"));
                            }
                        }
                        Ok(msg)
                    } else {
                        Ok(String::from("..."))
                    }
                })
        });

        Ok(Box::new(mapped))
    }

    async fn remove_image(&self, id: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        let options = bollard::query_parameters::RemoveImageOptionsBuilder::default()
            .force(true)
            .build();
        docker
            .remove_image(id, Some(options), None)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::DockerApi(format!("Cannot remove image: {e}")))
    }

    async fn tag_image(&self, id: &str, repo: &str, tag: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        let options = bollard::query_parameters::TagImageOptionsBuilder::default()
            .repo(repo)
            .tag(tag)
            .build();
        docker
            .tag_image(id, Some(options))
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot tag image: {e}")))
    }

    async fn inspect_image(&self, id: &str) -> DomainResult<String> {
        let docker = self.get_docker().await?;
        let info = docker
            .inspect_image(id)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot inspect image: {e}")))?;
        serde_json::to_string_pretty(&info)
            .map_err(|e| DomainError::Serialization(format!("Cannot format inspect result: {e}")))
    }
}

pub(crate) fn format_bytes(bytes: i64) -> String {
    if bytes < 0 {
        return "N/A".to_string();
    }
    let b = bytes as u64;
    if b < 1024 {
        format!("{b} B")
    } else if b < 1024 * 1024 {
        format!("{:.1} KB", b as f64 / 1024.0)
    } else if b < 1024 * 1024 * 1024 {
        format!("{:.1} MB", b as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", b as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_timestamp(unix: i64) -> String {
    chrono::DateTime::from_timestamp(unix, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}
