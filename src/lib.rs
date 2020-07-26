//! # ipfsapi
//! This is a crate for interfacing with the local IPFS API. It allows you to
//! read and write data to the IPFS network.

#[macro_use]
extern crate serde_derive;

mod v0;
mod ipns;
mod object;
pub mod pin;
pub mod pubsub;
mod version;
mod shutdown;
mod block;

pub struct IpfsApi {
    url: url::Url,
    client : reqwest::Client,
}

/// The main interface of the library
/// The IpfsApi class represents a connection to the local IPFS daemon. It can
/// read and write data to it using http requests to the server.
impl IpfsApi {
    /// Creates a new instance of the API
    ///
    /// ```rust,ignore
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// ```
    pub fn new(url: &str) -> Result<IpfsApi, url::ParseError> {
        Ok(IpfsApi {
            client: reqwest::Client::new(),
            url: url::Url::parse(url)?,
        })
    }

    /// Returns a Reqwest URL for the server
    /// Defaults to HTTP with no paths and no request parts.
    fn get_url(&self) -> Result<reqwest::Url, url::ParseError> {
        Ok(self.url.clone())
    }
}

