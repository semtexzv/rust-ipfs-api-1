use crate::IpfsApi;

use reqwest;
use failure::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct IpfsVersion {
    version: String,
    commit: String,
    repo: String,
    system: String,
    golang: String
}

impl IpfsApi {
    /// Get the version from the IPFS daemon.
    pub async fn version(&self) -> Result<IpfsVersion, Error> {
        let url = format!("http://{}:{}/api/v0/version", self.server, self.port);
        let resp = reqwest::get(&url).await?.json().await?;
        Ok(resp)
    }
}
