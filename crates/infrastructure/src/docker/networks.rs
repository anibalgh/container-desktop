use super::DockerClient;
use async_trait::async_trait;
use domain::entities::Network;
use domain::repository::NetworkRepository;
use domain::{DomainError, DomainResult};

#[async_trait]
impl NetworkRepository for DockerClient {
    async fn list_networks(&self) -> DomainResult<Vec<Network>> {
        let docker = self.get_docker().await?;
        let options = bollard::query_parameters::ListNetworksOptionsBuilder::default().build();
        let networks = docker
            .list_networks(Some(options))
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot list networks: {e}")))?;

        let result = networks
            .into_iter()
            .map(|n| {
                let subnet = n
                    .ipam
                    .as_ref()
                    .and_then(|i| i.config.as_ref())
                    .and_then(|c| c.first())
                    .and_then(|c| c.subnet.clone());
                let gateway = n
                    .ipam
                    .as_ref()
                    .and_then(|i| i.config.as_ref())
                    .and_then(|c| c.first())
                    .and_then(|c| c.gateway.clone());

                Network {
                    id: n.id.unwrap_or_default(),
                    name: n.name.unwrap_or_default(),
                    driver: n.driver.unwrap_or_default(),
                    scope: n.scope.unwrap_or_default(),
                    subnet,
                    gateway,
                    internal: n.internal.unwrap_or(false),
                    containers_count: 0,
                    created: n.created.map(|dt| dt.to_string()).unwrap_or_default(),
                }
            })
            .collect();

        Ok(result)
    }

    async fn create_network(&self, name: &str, driver: Option<&str>) -> DomainResult<String> {
        let docker = self.get_docker().await?;
        let config = bollard::config::NetworkCreateRequest {
            name: name.to_string(),
            driver: driver.map(|d| d.to_string()),
            ..Default::default()
        };
        let r = docker
            .create_network(config)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot create network: {e}")))?;
        Ok(r.id)
    }

    async fn remove_network(&self, id: &str) -> DomainResult<()> {
        let docker = self.get_docker().await?;
        docker
            .remove_network(id)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot remove network: {e}")))
    }

    async fn inspect_network(&self, id: &str) -> DomainResult<String> {
        let docker = self.get_docker().await?;
        let info = docker
            .inspect_network(id, None::<bollard::query_parameters::InspectNetworkOptions>)
            .await
            .map_err(|e| DomainError::DockerApi(format!("Cannot inspect network: {e}")))?;
        serde_json::to_string_pretty(&info)
            .map_err(|e| DomainError::Serialization(format!("Format error: {e}")))
    }
}
