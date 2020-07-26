use crate::IpfsApi;

use reqwest;
use anyhow::Error;

impl IpfsApi {
    /// Shut down the IPFS daemon
    /// This function causes the IPFS daemon to terminate
    pub async fn shutdown(&self) -> Result<(), Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/shutdown");
        let _resp = reqwest::get(url).await?;
        Ok(())
    }
}
