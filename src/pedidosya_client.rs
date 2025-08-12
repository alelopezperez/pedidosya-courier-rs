use reqwest::header::{self, AUTHORIZATION};
use serde::{Deserialize, de::Error as _};

use crate::models::{ContentType, Error, ResponseContent};

const PEDIDOSYA_BASE_URL: &str = "https://courier-api.pedidosya.com";

pub struct PedidosYaClient {
    client: reqwest::Client,
    base_path: String,
    auth_token: String,
}

impl PedidosYaClient {
    pub fn new<S: Into<String>>(auth_token: S) -> Self {
        let auth_token: String = auth_token.into();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            header::HeaderValue::from_str(auth_token.as_str())
                .expect("Error: could not parse auth token into a valid request header."),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Error: could not initialize PedidosYa API client. Please try again!");

        Self {
            client,
            base_path: PEDIDOSYA_BASE_URL.to_owned(),
            auth_token,
        }
    }

    pub async fn send_post_request<Req, Res, E>(
        &self,
        url: &str,
        body: &Req,
    ) -> Result<Res, Error<E>>
    where
        Req: serde::Serialize,
        Res: serde::de::DeserializeOwned,
        E: serde::de::DeserializeOwned,
    {
        let response = self.client.post(url).json(body).send().await?;
        let content_type: ContentType = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .into();
        let status = response.status();

        if response.status().is_success() {
            match content_type {
                ContentType::Json => Ok(response.json().await?),
                ContentType::Pdf => Err(Error::Serde(serde_json::Error::custom(
                    "Received `application/pdf` content type response that cannot be converted to `models::`",
                ))),
                ContentType::Unsoported(txt) => {
                    Err(Error::Serde(serde_json::Error::custom(format!(
                        "Received `{txt}` content type response that cannot be converted to `models::`"
                    ))))
                }
                ContentType::None => Err(Error::Serde(serde_json::Error::custom(
                    "Received empty content type response that cannot be converted to `models::`",
                ))),
            }
        } else if let ContentType::Json = content_type {
            let content = response.text().await?;
            let entity = serde_json::from_str(&content)?;
            let content = ResponseContent {
                status,
                content,
                entity,
            };
            Err(Error::ResponseError(content))
        } else {
            Err(Error::Serde(serde_json::Error::custom(
                "Received empty content type response that cannot be converted to `models::`",
            )))
        }
    }
}
