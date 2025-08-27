use crate::{StorefrontClient, ApiError, types::{Cart, CreateCartBody, MutateCartBody}};
impl StorefrontClient {
    /// POST /v1/carts (currency optional via query)
    pub async fn create_cart(
        &self,
        body: &CreateCartBody,
        currency: Option<&str>,
    ) -> Result<Cart, ApiError> {
        let url = self.url("carts")?;
        let qp = currency.map(|c| vec![("currency", c.to_string())]).unwrap_or_default();
        self.post_json(url, &qp, body).await
    }

    /// GET /v1/carts/{id}
    pub async fn cart(&self, id: &str) -> Result<Cart, ApiError> {
        let url = self.url(&format!("carts/{id}"))?;
        self.get(url, &[]).await
    }

    /// POST /v1/carts/{id}/add
    pub async fn cart_add(&self, id: &str, body: &MutateCartBody) -> Result<Cart, ApiError> {
        let url = self.url(&format!("carts/{id}/add"))?;
        self.post_json(url, &[], body).await
    }

    /// POST /v1/carts/{id}/remove
    pub async fn cart_remove(&self, id: &str, body: &MutateCartBody) -> Result<Cart, ApiError> {
        let url = self.url(&format!("carts/{id}/remove"))?;
        self.post_json(url, &[], body).await
    }

    /// POST /v1/carts/{id}/change
    pub async fn cart_change(&self, id: &str, body: &MutateCartBody) -> Result<Cart, ApiError> {
        let url = self.url(&format!("carts/{id}/change"))?;
        self.post_json(url, &[], body).await
    }
}
