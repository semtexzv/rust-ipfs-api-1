use bytes::Bytes;
use failure::Error;

#[derive(Deserialize)]
struct AddResponse {
    #[serde(rename = "Hash")]
    hash: String,
}

impl crate::IpfsApi {
    pub async fn add(&self, data: Bytes) -> Result<String, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/add");

        let res = {
            self.client.post(url)
                .multipart(reqwest::multipart::Form::new()
                    .part("arg", reqwest::multipart::Part::bytes(data.to_vec()))
                ).send().await?
        };

        let json: AddResponse = res.json().await?;
        Ok(json.hash)
    }

    /// Retrieves the contents of a file from the IPFS network. Takes a
    /// hash and returns an iterator of bytes. The result can be streamed, if
    /// the file is large.
    ///
    /// ```rust,ignore
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    ///
    /// let hello = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")?;
    /// let hello_string = String::from_utf8(hello.collect())?;
    /// println!("{}", hello_string);
    /// ```
    pub async fn cat(&self, hash: &str) -> Result<Bytes, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/cat");
        url.query_pairs_mut()
            .append_pair("arg", hash);
        let resp = self.client.post(url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?)
    }

    pub async fn get(&self, hash: &str) -> Result<Bytes, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/get");
        url.query_pairs_mut()
            .append_pair("arg", hash);
        let resp = self.client.post(url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?)
    }
}