use crate::{StorefrontClient, ApiError, types::{Page, Collection, Product}};
impl StorefrontClient {
    /// GET /v1/collections?page&size
    pub async fn collections(&self, page: i32, size: i32) -> Result<Page<Collection>, ApiError> {
        let url = self.url("collections")?;
        self.get(url, &[("page", page.to_string()), ("size", size.to_string())]).await
    }

    /// GET /v1/collections/{slug}
    pub async fn collection(&self, slug: &str) -> Result<Collection, ApiError> {
        let url = self.url(&format!("collections/{slug}"))?;
        self.get(url, &[]).await
    }

    /// GET /v1/collections/{slug}/products?currency=USD&page=0&size=20
    pub async fn collection_products(
        &self,
        slug: &str,
        currency: Option<&str>,
        page: i32,
        size: i32,
    ) -> Result<Page<Product>, ApiError> {
        let mut qp = vec![("page", page.to_string()), ("size", size.to_string())];
        if let Some(c) = currency { qp.push(("currency", c.to_string())); }
        let url = self.url(&format!("collections/{slug}/products"))?;
        self.get(url, &qp).await
    }
}
