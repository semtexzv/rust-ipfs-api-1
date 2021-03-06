use crate::IpfsApi;

use reqwest;
use serde_json;
use anyhow::{Error, anyhow};

#[derive(Deserialize, Debug, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct PinResponse {
    pins: Vec<String>
}

#[derive(PartialEq)]
pub enum PinType {
    Direct,
    Indirect,
    Recursive,
}

pub struct PinnedHash {
    pub hash: String,
    pub pin_type: PinType,
}

impl IpfsApi {
    /// Tells the IPFS server to pin the given object.
    /// If 'recursive' is true, it will recursively pin all objects
    /// that one depends on.
    /// If 'progress' is true, it will return a percentage(?) progress
    /// if the object has not been already pinned, or None if it has.
    pub async fn pin_add(&self, hash: &str, recursive: bool) -> Result<PinResponse, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/add");
        url.query_pairs_mut()
            .append_pair("arg", hash)
            .append_pair("recursive", &recursive.to_string())
            .append_pair("progress", "false");

        Ok(reqwest::get(url).await?.json().await?)
    }

    /// Unpin the given object.
    pub async fn pin_rm(&self, hash: &str, recursive: bool) -> Result<PinResponse, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/rm");
        url.query_pairs_mut()
            .append_pair("arg", hash)
            .append_pair("recursive", &recursive.to_string());

        Ok(reqwest::get(url).await?.json().await?)
    }


    /// List pinned objects.
    pub async fn pin_list(&self) -> Result<Vec<PinnedHash>, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/ls");
        let resp: serde_json::Value = reqwest::get(url).await?.json().await?;

        let mut hashes = Vec::new();

        let keys = resp.get("Keys").ok_or(anyhow!(""))?.as_object().ok_or(anyhow!(""))?;

        for (key, value) in keys.iter() {
            hashes.push(PinnedHash {
                hash: key.clone(),
                pin_type: match &value.get("Type").ok_or(anyhow!(""))?.as_str().ok_or(anyhow!(""))? {
                    &"direct" => PinType::Direct,
                    &"indirect" => PinType::Indirect,
                    &"recursive" => PinType::Recursive,
                    _ => PinType::Direct
                },
            });
        }

        Ok(hashes)
    }
}


#[cfg(test)]
mod tests {
    use crate::IpfsApi;
    use pin::PinType;

    // Add a pin, list it and then remove it.
    #[test]
    fn test_pin_full() {
        let api = IpfsApi::new("127.0.0.1", 5001);

        // Hello world object
        let hello = "QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u";

        // Unpin everything first
        for pin in api.pin_list().unwrap() {
            if pin.pin_type == PinType::Direct || pin.pin_type == PinType::Recursive {
                api.pin_rm(&pin.hash, true).unwrap();
            }
        }

        // Add pin
        let resp = api.pin_add(hello, false).unwrap();

        // Check if pin is added
        assert_eq!(resp.pins.len(), 1);
        assert_eq!(resp.pins[0], "QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u".to_string());
    }
}
