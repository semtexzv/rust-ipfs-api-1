use crate::IpfsApi;

use reqwest;
use failure::Error;

use bytes::Bytes;

#[derive(Deserialize)]
struct BlockPutResponse {
    #[serde(rename = "Key")]
    key: String,
}

impl IpfsApi {
    pub async fn block_put<'a>(&'a self, data: Bytes) -> Result<String, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/block/put");

        let res = {
            self.client.post(url)
                .multipart(reqwest::multipart::Form::new()
                    .part("arg", reqwest::multipart::Part::bytes(data.to_vec()))
                ).send().await?
        };

        let json: BlockPutResponse = res.json().await?;
        Ok(json.key)
    }

    pub async fn block_get(&self, hash: &str) -> Result<Bytes, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/block/get");
        url.query_pairs_mut()
            .append_pair("arg", hash);

        Ok(self.client.get(url).send().await?.bytes().await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::IpfsApi;

    #[test]
    fn test_block_put() {
        let api = IpfsApi::new("127.0.0.1", 5001);

        let hash = api.block_put("Hello world".as_bytes()).unwrap();

        assert_eq!(hash, "QmV8cfu6n4NT5xRr2AHdKxFMTZEJrA44qgrBCr739BN9Wb");
    }

    #[test]
    fn test_block_get() {
        let api = IpfsApi::new("127.0.0.1", 5001);

        let content = "Hello world\n".as_bytes();

        let hash = api.block_put(content).unwrap();
        let block: Vec<u8> = api.block_get(&hash).unwrap().collect();

        assert_eq!(block, content);
    }
}