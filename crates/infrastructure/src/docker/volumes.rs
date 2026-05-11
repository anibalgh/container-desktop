use super::DockerClient;
use async_trait::async_trait;
use bollard::query_parameters::ListVolumesOptionsBuilder;
use domain::entities::Volume;
use domain::repository::VolumeRepository;
use domain::{DomainError, DomainResult};

#[async_trait]
impl VolumeRepository for DockerClient {
    async fn list_volumes(&self) -> DomainResult<Vec<Volume>> {
        let docker = self.get_docker().await?;

        let options = ListVolumesOptionsBuilder::default().build();

        let response = docker
            .list_volumes(Some(options))
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot list volumes: {e}")))?;

        let result = response
            .volumes
            .into_iter()
            .flatten()
            .map(|v| Volume {
                name: v.name,
                driver: v.driver,
                mountpoint: v.mountpoint,
                scope: v.scope.map(|s| format!("{s:?}")).unwrap_or_default(),
                created: v
                    .created_at
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default(),
            })
            .collect();

        Ok(result)
    }

    async fn create_volume(&self, name: &str) -> DomainResult<Volume> {
        let docker = self.get_docker().await?;

        let config = bollard::config::VolumeCreateRequest {
            name: Some(name.to_string()),
            driver: Some("local".to_string()),
            ..Default::default()
        };

        let volume = docker
            .create_volume(config)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot create volume: {e}")))?;

        Ok(Volume {
            name: volume.name,
            driver: volume.driver,
            mountpoint: volume.mountpoint,
            scope: volume.scope.map(|s| format!("{s:?}")).unwrap_or_default(),
            created: volume
                .created_at
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default(),
        })
    }

    async fn remove_volume(&self, name: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .remove_volume(name, None::<bollard::query_parameters::RemoveVolumeOptions>)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot remove volume: {e}")))
    }

    async fn inspect_volume(&self, name: &str) -> DomainResult<String> {
        let docker = self.get_docker().await?;
        let info = docker
            .inspect_volume(name)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot inspect volume: {e}")))?;
        serde_json::to_string_pretty(&info)
            .map_err(|e| DomainError::Serialization(format!("Cannot format inspect result: {e}")))
    }
}
