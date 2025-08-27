use crate::{StorefrontClient, ApiError, types::Product};

impl StorefrontClient {
    /// GET /v1/products/{slug}
    pub async fn product(&self, slug: &str) -> Result<Product, ApiError> {
        let url = self.url(&format!("products/{slug}"))?;
        self.get(url, &[]).await
    }
}
