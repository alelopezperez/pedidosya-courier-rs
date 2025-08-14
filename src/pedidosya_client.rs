use reqwest::{
    Request,
    header::{self, AUTHORIZATION},
};
use serde::{Deserialize, Serialize, de::Error as _};

use crate::models::{
    ConfirmEstimationShippingRequest, ConfirmShippingResponse, ContentType, Error,
    EstimationShippingResponse, HttpErrorResponse, ResponseContent,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfirmEstimateError {
    Status400(HttpErrorResponse),
    Status403(HttpErrorResponse),
    Status409(HttpErrorResponse),
    Status500(HttpErrorResponse),
    StatusNonExpected(HttpErrorResponse),
    UnknownValue(serde_json::Value),
}
impl From<HttpErrorResponse> for ConfirmEstimateError {
    fn from(value: HttpErrorResponse) -> Self {
        match value.status {
            Some(400) => Self::Status400(value),
            Some(403) => Self::Status403(value),
            Some(409) => Self::Status409(value),
            Some(500) => Self::Status500(value),
            Some(_) => Self::StatusNonExpected(value),
            None => Self::StatusNonExpected(value),
        }
    }
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

    async fn send_post_request<Res, E>(&self, request: Request) -> Result<Res, Error<E>>
    where
        Res: serde::de::DeserializeOwned,
        E: serde::de::DeserializeOwned + From<HttpErrorResponse>,
    {
        let response = self.client.execute(request).await?;

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
            let entity: Option<E> = serde_json::from_str::<HttpErrorResponse>(&content)
                .ok()
                .map(E::from);

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
        let url_path = "/v3/shippings/estimates";
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                format!("{}{}", self.base_path, url_path),
            )
            .json(&estimation_shipping_request)
            .build()?;

        self.send_post_request(request).await
    }

    pub async fn shipping_confirm_estimate_order(
        &self,
        estimate_id: impl Into<String>,
        confirm_estimate_request: ConfirmEstimationShippingRequest,
    ) -> Result<ConfirmShippingResponse, Error<ConfirmEstimateError>> {
        let url_path = format!("/v3/shippings/estimates/{}/confirm", estimate_id.into());
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                format!("{}{}", self.base_path, url_path),
            )
            .json(&confirm_estimate_request)
            .build()?;

        self.send_post_request(request).await
    }
}

pub enum WebhookGetConfigurationError {
    Status403(HttpErrorResponse),
}
pub mod webhooks_blocking {
    use crate::models::{Error as PedidosError, WebhooksConfigModel};

    pub fn blocking_webhook_get_webhooks_configuration(
        api_key: String,
    ) -> Result<WebhooksConfigModel, PedidosError<()>> {
        let uri = "https://courier-api.pedidosya.com/v3/webhooks-configuration";

        let client = reqwest::blocking::Client::new();

        client
            .get(uri)
            .header(reqwest::header::AUTHORIZATION, api_key)
            .send()
            .inspect(|r| println!("{}", r.status()))
            .and_then(|r| r.json::<WebhooksConfigModel>())
            .map_err(PedidosError::from)
    }
    pub fn blocking_webhook_set_webhooks_configuration(
        api_key: String,

        webhook_config_request: WebhooksConfigModel,
    ) -> Result<WebhooksConfigModel, PedidosError<()>> {
        let uri = "https://courier-api.pedidosya.com/v3/webhooks-configuration";

        let client = reqwest::blocking::Client::new();

        client
            .put(uri)
            .header(reqwest::header::AUTHORIZATION, api_key)
            .json(&webhook_config_request)
            .send()
            .and_then(|res| res.json::<WebhooksConfigModel>())
            .map_err(PedidosError::from)
    }
}
#[cfg(test)]
mod tests {
    use crate::models::WebhooksConfigModel;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_serialization() {
        let j = "
        {
            \"status\": 403,
            \"message\": \"Menlo Park, CA\",
            \"code\": \"Menlo Park, CA\"
        }";

        let v = serde_json::from_str::<GetShippingsEstimatesError>(j).unwrap();

        println!("{:?}", v)
    }

    #[test]
    fn test_serialization_2() {
        let j = "
        {
            \"status\": 403,
            \"message\": \"Menlo Park, CA\",
            \"code\": \"Menlo Park, CA\"
        }";

        let v = serde_json::from_str::<WebhooksConfigModel>(j);

        println!("{:?}", v)
    }
}
