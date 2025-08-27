use crate::error::ApiError;
use reqwest::{Client as HttpClient, Url};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct StorefrontClient {
    http: HttpClient,
    base: Url,
    token: String,
}

impl StorefrontClient {
    /// `token` is the Storefront token (ptkn_xxx). It is sent as `storefront_token` query param.
    pub fn new(token: impl Into<String>) -> Result<Self, ApiError> {
        let builder = reqwest::Client::builder().user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));
        #[cfg(not(target_arch = "wasm32"))]
        let builder = builder.timeout(std::time::Duration::from_secs(15));
        let http = builder.build()?;

        Ok(Self {
            http: http,
            base: Url::parse("https://storefront-api.fourthwall.com/v1/")?,
            token: token.into(),
        })
    }

    pub(crate) fn url(&self, path: &str) -> Result<Url, ApiError> {
        Ok(self.base.join(path)?)
    }

    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        url: Url,
        qp: &[(&str, String)],
    ) -> Result<T, ApiError> {
        let mut pairs = vec![("storefront_token", self.token.clone())];
        pairs.extend(qp.iter().cloned().map(|(k, v)| (k, v)));
        let res = self.http.get(url).query(&pairs).send().await?;
        crate::error::map_json(res).await
    }

    pub(crate) async fn post_json<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        url: Url,
        qp: &[(&str, String)],
        body: &B,
    ) -> Result<T, ApiError> {
        let mut pairs = vec![("storefront_token", self.token.clone())];
        pairs.extend(qp.iter().cloned().map(|(k, v)| (k, v)));
        let res = self.http.post(url).query(&pairs).json(body).send().await?;
        crate::error::map_json(res).await
    }
}
