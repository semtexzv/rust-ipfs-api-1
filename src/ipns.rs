use crate::IpfsApi;

use serde_json::Value;

use reqwest;
use serde_json;
use anyhow::{Error, anyhow};

impl IpfsApi {
    /// Resolve an IPNS hash or a domain name
    ///
    /// ```rust,ignore
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// let hash = api.name_resolve("gkbrk.com");
    ///
    /// println!("{}", hash);
    /// ```
    pub async fn name_resolve(&self, name: &str) -> Result<String, Error> {
        let url = format!("{}/api/v0/name/resolve?arg={}", self.url, name);
        let resp: Value = reqwest::get(&url).await?.json().await?;

        if resp["Path"].is_string() {
            Ok(resp["Path"].as_str().unwrap().into())
        } else {
            Err(anyhow!("Key error"))
        }
    }

    /// Publish an IPFS hash in IPNS.
    pub async fn name_publish(&self, hash: &str) -> Result<(), Error> {
        let url = format!("{}/api/v0/name/publish?arg={}", self.url, hash);
        let _resp = reqwest::get(&url).await?;
        Ok(())
    }
}
