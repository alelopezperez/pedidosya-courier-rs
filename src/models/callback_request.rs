use crate::models;
use serde::{Deserialize, Serialize};

/// CallbackRequest : Callback Request We could have different topics. Right now we only provide SHIPPING_STATUS but more topics will be available in future.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackRequest {
    /// Topic suscription name.
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<Topic>,
    /// Shipping identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Client Internal Reference ID
    #[serde(rename = "referenceId", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// Date time (in UTC) when the message was generated. For example, 2020-07-21T17:32:28Z. Format ISO 8601: YYYY-MM-DDTHH:MM:SSZ
    #[serde(rename = "generated", skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
    /// Date time (in UTC) when the message was transmitted. For example, 2020-07-21T17:32:28Z. Format ISO 8601: YYYY-MM-DDTHH:MM:SSZ
    #[serde(rename = "transmitted", skip_serializing_if = "Option::is_none")]
    pub transmitted: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<CallbackRequestData>>,
}

/// Topic suscription name.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Topic {
    #[serde(rename = "SHIPPING_STATUS")]
    ShippingStatus,
}

impl Default for Topic {
    fn default() -> Topic {
        Self::ShippingStatus
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackRequestData {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CallbackShippingStatus>,
    #[serde(rename = "cancelCode", skip_serializing_if = "Option::is_none")]
    pub cancel_code: Option<CancelCode>,
    /// Spanish message text for the cancel code. This field is setted only if status is CANCELLED.
    #[serde(rename = "cancelReason", skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    /// Estimation pickup time (in UTC). This time is dynamically updated. For example, 2020-07-21T17:32:28Z. Format ISO 8601: YYYY-MM-DDTHH:MM:SSZ. This field is set only if the status is: CONFIRMED, IN_PROGRESS (transport has been assigned) or NEAR_PICKUP.
    #[serde(
        rename = "estimatedPickUpTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_pick_up_time: Option<String>,
    /// Estimation drop off time (in UTC). This time is dynamically updated. For example, 2020-07-21T17:32:28Z. Format ISO 8601: YYYY-MM-DDTHH:MM:SSZ. This field is set only if the status is CONFIRMED, IN_PROGRESS (transport has been assigned), NEAR_PICKUP, PICKED_UP, NEAR_DROPOFF.
    #[serde(
        rename = "estimatedDropOffTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_drop_off_time: Option<String>,
}

impl CallbackRequestData {
    /// Extra information depending on topic.  * SHIPPING_STATUS: returns property 'status', and if status is CANCELLED it returns 'cancelCode' and 'cancelReason' also.
    pub fn new() -> CallbackRequestData {
        CallbackRequestData {
            status: None,
            cancel_code: None,
            cancel_reason: None,
            estimated_pick_up_time: None,
            estimated_drop_off_time: None,
        }
    }
}

/// CallbackShippingStatus : We only send callback shipping status for this status values:  * Confirmed: Shipping order confirmed and awaiting for dispatching  * In Progress: Transport has been assigned   * Near Pickup: Transport is near pickup point  * Picked up: Transport picked up the order's items  * Near Dropoff: Transport is closest to dropoff point  * Completed: Transport had delivered the items  * Cancelled: Shipping order cancelled for any reason.
/// We only send callback shipping status for this status values:  * Confirmed: Shipping order confirmed and awaiting for dispatching  * In Progress: Transport has been assigned   * Near Pickup: Transport is near pickup point  * Picked up: Transport picked up the order's items  * Near Dropoff: Transport is closest to dropoff point  * Completed: Transport had delivered the items  * Cancelled: Shipping order cancelled for any reason.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallbackShippingStatus {
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "NEAR_PICKUP")]
    NearPickup,
    #[serde(rename = "PICKED_UP")]
    PickedUp,
    #[serde(rename = "NEAR_DROPOFF")]
    NearDropoff,
    #[serde(rename = "COMPLETED")]
    Completed,
}

impl std::fmt::Display for CallbackShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Confirmed => write!(f, "CONFIRMED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::InProgress => write!(f, "IN_PROGRESS"),
            Self::NearPickup => write!(f, "NEAR_PICKUP"),
            Self::PickedUp => write!(f, "PICKED_UP"),
            Self::NearDropoff => write!(f, "NEAR_DROPOFF"),
            Self::Completed => write!(f, "COMPLETED"),
        }
    }
}

impl Default for CallbackShippingStatus {
    fn default() -> CallbackShippingStatus {
        Self::Confirmed
    }
}

/// CancelCode : This field is setted only if status is CANCELLED. The Cancel Reason Codes are show bellow with its spanish messages for cancelReason field: * ADDRESS_DATA_MISSING: Rider no encuentra el pickup/dropoff  * NO_RIDER_AVAILABLE: No hay cadete disponible en este momento  * OUT_OF_DELIVERY_ZONE: Fuera de área de cobertura del servicio  * DELAYED_DELIVERY_SCHEDULE: Cancelado debido a horario de entrega retrasado  * COORDINATE_ERROR: Coordenadas no concuerdan con la dirección ingresada  * PACKAGE_DAMAGE_LOOSE: Se produjo un problema con el producto o paquete  * ORDER_NOT_DELIVERED: Pedido no entregado  * INAPPROPRIATE_CONDUCT: Cancelado por problemas con el rider  * UNREACHABLE_RIDER: Cancelado por problemas con el rider  * TYC_PACKAGE_CONTRADICTION: Pedido incorrecto. Paquete o producto no respeta TyC.  * PURCHASE_REQUESTED: Pedido realizado por error  * USER_CANNOT_PAY: Solicitud de envío pendiente de pago. El usuario no puede pagar el pedido.  * COUPON_NOT_APPLIED: No fue posible aplicar el cupón.  * DUPLICATED_ORDER: Pedido duplicado  * UNREACHABLE_USER_DROPOFF: No es posible contactar al cliente en Punto de Entrega  * SUSPICIOUS_CLIENT: Pedido incorrecto. * USER_CANCELLED: Cancelado a solicitud del usuario  * TECHNICAL_PROBLEM: Cancelado por problemas técnicos  * BAD_WEATHER: Condiciones climáticas adversas  * UNREACHABLE_USER_PICKUP: No es posible contactar al cliente en Punto de Retiro  * CONTENT_WRONG: Producto despachado no es correcto.  * ORDER_MODIFICATION: No es posible modificar punto de origen o destino  * OUT_OF_FLEET_TIME: Fuera de horario de servicio  * TEST_ORDER: Orden de prueba - TEST  * CONTENT_WRONG_RIDER: Producto despachado no es correcto
/// This field is setted only if status is CANCELLED. The Cancel Reason Codes are show bellow with its spanish messages for cancelReason field: * ADDRESS_DATA_MISSING: Rider no encuentra el pickup/dropoff  * NO_RIDER_AVAILABLE: No hay cadete disponible en este momento  * OUT_OF_DELIVERY_ZONE: Fuera de área de cobertura del servicio  * DELAYED_DELIVERY_SCHEDULE: Cancelado debido a horario de entrega retrasado  * COORDINATE_ERROR: Coordenadas no concuerdan con la dirección ingresada  * PACKAGE_DAMAGE_LOOSE: Se produjo un problema con el producto o paquete  * ORDER_NOT_DELIVERED: Pedido no entregado  * INAPPROPRIATE_CONDUCT: Cancelado por problemas con el rider  * UNREACHABLE_RIDER: Cancelado por problemas con el rider  * TYC_PACKAGE_CONTRADICTION: Pedido incorrecto. Paquete o producto no respeta TyC.  * PURCHASE_REQUESTED: Pedido realizado por error  * USER_CANNOT_PAY: Solicitud de envío pendiente de pago. El usuario no puede pagar el pedido.  * COUPON_NOT_APPLIED: No fue posible aplicar el cupón.  * DUPLICATED_ORDER: Pedido duplicado  * UNREACHABLE_USER_DROPOFF: No es posible contactar al cliente en Punto de Entrega  * SUSPICIOUS_CLIENT: Pedido incorrecto. * USER_CANCELLED: Cancelado a solicitud del usuario  * TECHNICAL_PROBLEM: Cancelado por problemas técnicos  * BAD_WEATHER: Condiciones climáticas adversas  * UNREACHABLE_USER_PICKUP: No es posible contactar al cliente en Punto de Retiro  * CONTENT_WRONG: Producto despachado no es correcto.  * ORDER_MODIFICATION: No es posible modificar punto de origen o destino  * OUT_OF_FLEET_TIME: Fuera de horario de servicio  * TEST_ORDER: Orden de prueba - TEST  * CONTENT_WRONG_RIDER: Producto despachado no es correcto
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CancelCode {
    #[serde(rename = "ADDRESS_DATA_MISSING")]
    AddressDataMissing,
    #[serde(rename = "NO_RIDER_AVAILABLE")]
    NoRiderAvailable,
    #[serde(rename = "OUT_OF_DELIVERY_ZONE")]
    OutOfDeliveryZone,
    #[serde(rename = "DELAYED_DELIVERY_SCHEDULE")]
    DelayedDeliverySchedule,
    #[serde(rename = "COORDINATE_ERROR")]
    CoordinateError,
    #[serde(rename = "PACKAGE_DAMAGE_LOOSE")]
    PackageDamageLoose,
    #[serde(rename = "ORDER_NOT_DELIVERED")]
    OrderNotDelivered,
    #[serde(rename = "INAPPROPRIATE_CONDUCT")]
    InappropriateConduct,
    #[serde(rename = "UNREACHABLE_RIDER")]
    UnreachableRider,
    #[serde(rename = "TYC_PACKAGE_CONTRADICTION")]
    TycPackageContradiction,
    #[serde(rename = "PURCHASE_REQUESTED")]
    PurchaseRequested,
    #[serde(rename = "USER_CANNOT_PAY")]
    UserCannotPay,
    #[serde(rename = "COUPON_NOT_APPLIED")]
    CouponNotApplied,
    #[serde(rename = "DUPLICATED_ORDER")]
    DuplicatedOrder,
    #[serde(rename = "UNREACHABLE_USER_DROPOFF")]
    UnreachableUserDropoff,
    #[serde(rename = "SUSPICIOUS_CLIENT")]
    SuspiciousClient,
    #[serde(rename = "USER_CANCELLED")]
    UserCancelled,
    #[serde(rename = "TECHNICAL_PROBLEM")]
    TechnicalProblem,
    #[serde(rename = "BAD_WEATHER")]
    BadWeather,
    #[serde(rename = "UNREACHABLE_USER_PICKUP")]
    UnreachableUserPickup,
    #[serde(rename = "CONTENT_WRONG")]
    ContentWrong,
    #[serde(rename = "ORDER_MODIFICATION")]
    OrderModification,
    #[serde(rename = "OUT_OF_FLEET_TIME")]
    OutOfFleetTime,
    #[serde(rename = "TEST_ORDER")]
    TestOrder,
    #[serde(rename = "CONTENT_WRONG_RIDER")]
    ContentWrongRider,
}

impl std::fmt::Display for CancelCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AddressDataMissing => write!(f, "ADDRESS_DATA_MISSING"),
            Self::NoRiderAvailable => write!(f, "NO_RIDER_AVAILABLE"),
            Self::OutOfDeliveryZone => write!(f, "OUT_OF_DELIVERY_ZONE"),
            Self::DelayedDeliverySchedule => write!(f, "DELAYED_DELIVERY_SCHEDULE"),
            Self::CoordinateError => write!(f, "COORDINATE_ERROR"),
            Self::PackageDamageLoose => write!(f, "PACKAGE_DAMAGE_LOOSE"),
            Self::OrderNotDelivered => write!(f, "ORDER_NOT_DELIVERED"),
            Self::InappropriateConduct => write!(f, "INAPPROPRIATE_CONDUCT"),
            Self::UnreachableRider => write!(f, "UNREACHABLE_RIDER"),
            Self::TycPackageContradiction => write!(f, "TYC_PACKAGE_CONTRADICTION"),
            Self::PurchaseRequested => write!(f, "PURCHASE_REQUESTED"),
            Self::UserCannotPay => write!(f, "USER_CANNOT_PAY"),
            Self::CouponNotApplied => write!(f, "COUPON_NOT_APPLIED"),
            Self::DuplicatedOrder => write!(f, "DUPLICATED_ORDER"),
            Self::UnreachableUserDropoff => write!(f, "UNREACHABLE_USER_DROPOFF"),
            Self::SuspiciousClient => write!(f, "SUSPICIOUS_CLIENT"),
            Self::UserCancelled => write!(f, "USER_CANCELLED"),
            Self::TechnicalProblem => write!(f, "TECHNICAL_PROBLEM"),
            Self::BadWeather => write!(f, "BAD_WEATHER"),
            Self::UnreachableUserPickup => write!(f, "UNREACHABLE_USER_PICKUP"),
            Self::ContentWrong => write!(f, "CONTENT_WRONG"),
            Self::OrderModification => write!(f, "ORDER_MODIFICATION"),
            Self::OutOfFleetTime => write!(f, "OUT_OF_FLEET_TIME"),
            Self::TestOrder => write!(f, "TEST_ORDER"),
            Self::ContentWrongRider => write!(f, "CONTENT_WRONG_RIDER"),
        }
    }
}

impl Default for CancelCode {
    fn default() -> CancelCode {
        Self::AddressDataMissing
    }
}
