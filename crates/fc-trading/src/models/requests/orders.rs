use serde::Serialize;

use crate::DecimalNumber;

#[derive(Clone, Debug, Serialize)]
pub struct NewOrderRequest {
    pub account: String,
    #[serde(rename = "requestID")]
    pub request_id: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    pub market: String,
    #[serde(rename = "buySell")]
    pub buy_sell: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    pub price: DecimalNumber,
    pub quantity: i64,
    #[serde(rename = "stopOrder")]
    pub stop_order: bool,
    #[serde(rename = "stopPrice")]
    pub stop_price: DecimalNumber,
    #[serde(rename = "stopType")]
    pub stop_type: String,
    #[serde(rename = "stopStep")]
    pub stop_step: DecimalNumber,
    #[serde(rename = "lossStep")]
    pub loss_step: DecimalNumber,
    #[serde(rename = "profitStep")]
    pub profit_step: DecimalNumber,
    #[serde(rename = "channelID")]
    pub channel_id: String,
    pub code: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiable: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CancelOrderRequest {
    pub account: String,
    #[serde(rename = "requestID")]
    pub request_id: String,
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(rename = "marketID")]
    pub market_id: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    #[serde(rename = "buySell")]
    pub buy_sell: String,
    pub code: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModifyOrderRequest {
    pub account: String,
    #[serde(rename = "requestID")]
    pub request_id: String,
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(rename = "marketID")]
    pub market_id: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    pub price: DecimalNumber,
    pub quantity: i64,
    #[serde(rename = "buySell")]
    pub buy_sell: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    pub code: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}

pub type DerivativeNewOrderRequest = NewOrderRequest;
pub type DerivativeCancelOrderRequest = CancelOrderRequest;
pub type DerivativeModifyOrderRequest = ModifyOrderRequest;
