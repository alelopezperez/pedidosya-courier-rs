use reqwest::header::{self, AUTHORIZATION};
use serde::{Deserialize, Serialize, de::Error as _};

use crate::models::{
    ContentType, Error, EstimationShippingResponse, HttpErrorResponse, ResponseContent,
    estimation_shipping_request::EstimationShippingRequest,
};

const PEDIDOSYA_BASE_URL: &str = "https://courier-api.pedidosya.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShippingsEstimatesError {
    Status400(HttpErrorResponse),
    Status403(HttpErrorResponse),
    Status500(HttpErrorResponse),
    StatusNonExpected(HttpErrorResponse),
    UnknownValue(serde_json::Value),
}

impl From<HttpErrorResponse> for GetShippingsEstimatesError {
    fn from(value: HttpErrorResponse) -> Self {
        match value.status {
            Some(400) => Self::Status400(value),
            Some(403) => Self::Status403(value),
            Some(500) => Self::Status500(value),
            Some(_) => Self::StatusNonExpected(value),
            None => Self::StatusNonExpected(value),
        }
    }
}

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
        E: serde::de::DeserializeOwned + From<HttpErrorResponse>,
    {
        let response = self
            .client
            .post(format!("{}/{}", self.base_path, url))
            .json(body)
            .send()
            .await?;

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
            let http_error_response: HttpErrorResponse = serde_json::from_str(&content)?;
            let entity = Some(E::from(http_error_response));
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

    pub async fn shipping_estimate_shipping_order(
        &self,
        estimation_shipping_request: EstimationShippingRequest,
    ) -> Result<EstimationShippingResponse, Error<GetShippingsEstimatesError>> {
        self.send_post_request("/v3/shippings/estimates", &estimation_shipping_request)
            .await
    }
}
