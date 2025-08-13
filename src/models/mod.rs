pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}
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

pub mod estimation_shipping_request;
pub mod estimation_shipping_response;
pub mod http_errors;

mod delivery_offer;
mod route;
mod shipping_item_request;
mod shipping_pricing_route;
mod shipping_request_requirements;
mod way_point_model;
mod way_point_model_response;

pub use delivery_offer::DeliveryOffer;
pub use estimation_shipping_response::EstimationShippingResponse;
pub use http_errors::HttpErrorResponse;
pub use route::Route;
pub use shipping_item_request::ShippingItemRequest;
pub use shipping_pricing_route::ShippingRoutePricing;
pub use shipping_request_requirements::ShippingRequestRequirements;
pub use way_point_model::WayPointModel;
pub use way_point_model_response::WayPointModelResponse;
