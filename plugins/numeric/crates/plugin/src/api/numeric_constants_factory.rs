use async_trait::async_trait;
use reactive_graph_plugin_api::injectable;

#[async_trait]
#[injectable]
// #[async_trait]
pub trait NumericConstantsFactory: Send + Sync {
    async fn create_numeric_constants(&self);
}
