use anyhow::{anyhow, Result};

use jsonwebtoken::{crypto, Algorithm, DecodingKey, EncodingKey};
use reqwest::{header::HeaderMap, header::HeaderValue, Client, StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug)]
struct CloudflareStreams {
    client: Client,
    base: Url,
    account_identifier: String,
}

impl CloudflareStreams {
    #[tracing::instrument]
    pub fn try_new(base_url: Url, account_identifier: String, api_key: &str) -> Result<Self> {
        let mut api_key_value = HeaderValue::from_str(api_key)?;
        api_key_value.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", api_key_value);
        headers.insert(
            "content-type",
            HeaderValue::from_static("application/json;charset=UTF-8"),
        );
        Ok(Self {
            client: Client::builder().default_headers(headers).build()?,
            base: base_url,
            account_identifier,
        })
    }

    #[tracing::instrument]
    pub async fn get_tus_upload_url(&self, creator_name: &str, video_length: usize) -> Result<Url> {
        let endpoint = self
            .base
            .join("accounts")
            .and_then(|url| url.join(&self.account_identifier))
            .and_then(|url| url.join("stream"))?;
        let response = self
            .client
            .post(endpoint)
            .header("Tus-Resumable", "1.0.0")
            .header("Upload-Length", video_length)
            .header("Upload-Creator", creator_name)
            .send()
            .await?;
        let response = response.error_for_status()?;
        match (response.status(), response.headers().get("location")) {
            (StatusCode::CREATED, Some(location)) => {
                let location_str = location.to_str()?;
                let url = Url::parse(location_str)?;
                Ok(url)
            }
            _ => Err(anyhow!("Platform error")),
        }
    }

    #[tracing::instrument]
    pub async fn get_signing_keys(&self) -> Result<EncodingKey> {
        #[derive(Deserialize)]
        struct SigningKeys {
            pub _id: String,
            pub pem: String,
        }
        let endpoint = self
            .base
            .join("accounts")
            .and_then(|url| url.join(&self.account_identifier))
            .and_then(|url| url.join("stream/keys"))?;
        let response = self.client.post(endpoint).send().await?;
        let response: CloudflareResponse<SigningKeys> = response.error_for_status()?.json().await?;
        EncodingKey::from_rsa_pem(response.result.pem.as_bytes()).map_err(Into::into)
    }

    #[tracing::instrument]
    pub async fn get_video_list(&self, creator_name: &str) -> Result<Vec<VideoMetaData>> {
        todo!()
    }

    pub fn verify_webhook(
        &self,
        secret: &str,
        timestamp: i64,
        signature: &str,
        request_body: &[u8],
    ) -> Result<bool> {
        let key = DecodingKey::from_secret(secret.as_bytes());
        let mut message = Vec::new();
        let timestamp = timestamp.to_string();
        message.extend(timestamp.as_bytes().into_iter());
        message.extend(".".as_bytes().into_iter());
        message.extend(request_body.into_iter());
        crypto::verify(signature, &message, &key, Algorithm::HS256).map_err(Into::into)
    }
}

#[derive(Deserialize)]
struct CloudflareResponse<T> {
    pub result: T,
}

#[derive(Deserialize)]
struct VideoMetaData {
    pub video_id: String,
}
