use crate::models;
use serde::{Deserialize, Serialize};

/// ShippingResponse : This model represents a shipping estimate order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShippingResponse {
    /// Shipping estimate identifier
    #[serde(rename = "shippingId", skip_serializing_if = "Option::is_none")]
    pub shipping_id: Option<String>,
    /// Client Internal Reference ID
    #[serde(rename = "referenceId", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// If you are creating a TEST shipping, set this field true
    #[serde(rename = "isTest", skip_serializing_if = "Option::is_none")]
    pub is_test: Option<bool>,
    /// List of items shipped in the package
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::ShippingItemRequest>>,
    /// Geographical points where transport should go. There must be two waypoints, one with type PICK_UP & one DROP_OFF. The geographical points must be inside the PedidosYa fleet's working zone. You can obtain the working zones in the endpoint <a href='#tag/Coverage/paths/~1v3~1working-zones/get'>Get Working Zones</a>.
    #[serde(rename = "waypoints", skip_serializing_if = "Option::is_none")]
    pub waypoints: Option<Vec<models::WayPointModelResponse>>,
    /// Delivery offers available for the estimation. This field has data related to delivery type, price and times info.
    #[serde(rename = "deliveryOffers", skip_serializing_if = "Option::is_none")]
    pub delivery_offers: Option<Vec<models::DeliveryOffer>>,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Box<models::Route>>,
    /// This email will be used to send shipping confirmation and cancellation notifications to the end user.
    #[serde(rename = "notificationMail", skip_serializing_if = "Option::is_none")]
    pub notification_mail: Option<String>,
}

impl ShippingResponse {
    /// This model represents a shipping estimate order.
    pub fn new() -> ShippingResponse {
        ShippingResponse {
            shipping_id: None,
            reference_id: None,
            is_test: None,
            items: None,
            waypoints: None,
            delivery_offers: None,
            route: None,
            notification_mail: None,
        }
    }
}
