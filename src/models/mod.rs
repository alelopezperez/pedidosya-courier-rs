#[derive(Debug)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}
impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

pub enum ContentType {
    Json,
    Pdf,
    Unsoported(String),
    None,
}

impl From<Option<&str>> for ContentType {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(text) => {
                if text.starts_with("application/json") {
                    ContentType::Json
                } else if text.starts_with("application/pdf") {
                    ContentType::Pdf
                } else {
                    ContentType::Unsoported(text.to_owned())
                }
            }
            None => ContentType::None,
        }
    }
}

pub mod confirm_estime_order_request;
pub mod confirm_shipping_order_response;
pub mod estimation_shipping_request;
pub mod estimation_shipping_response;
pub mod http_errors;
pub mod shipping_route;
pub mod shipping_status;
pub mod urls;
pub mod webhook_configuration;
pub mod webhook_configuration_model;

mod delivery_offer;
mod route;
mod shipping_item_request;
mod shipping_pricing_route;
mod shipping_request_requirements;
mod way_point_model;
mod way_point_model_response;

pub use confirm_estime_order_request::ConfirmEstimationShippingRequest;
pub use confirm_shipping_order_response::ConfirmShippingResponse;
pub use delivery_offer::DeliveryOffer;
pub use estimation_shipping_request::EstimationShippingRequest;
pub use estimation_shipping_response::EstimationShippingResponse;
pub use http_errors::HttpErrorResponse;
pub use route::Route;
pub use shipping_item_request::ShippingItemRequest;
pub use shipping_pricing_route::ShippingRoutePricing;
pub use shipping_request_requirements::ShippingRequestRequirements;
pub use shipping_route::ShippingRoute;
pub use shipping_status::ShippingStatus;
pub use urls::Urls;
pub use way_point_model::Type as WayPointModelType;
pub use way_point_model::WayPointModel;
pub use way_point_model_response::WayPointModelResponse;
pub use webhook_configuration::WebhookConfiguration;
pub use webhook_configuration_model::WebhooksConfigModel;
