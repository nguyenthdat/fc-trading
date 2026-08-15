use serde::Deserialize;

use crate::DecimalNumber;

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct MaxBuyQuantityData {
    pub account: String,
    #[serde(rename = "maxBuyQty")]
    pub max_buy_quantity: i64,
    #[serde(rename = "marginRatio")]
    pub margin_ratio: String,
    #[serde(rename = "purchasingPower")]
    pub purchasing_power: DecimalNumber,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct MaxSellQuantityData {
    pub account: String,
    #[serde(rename = "maxSellQty")]
    pub max_sell_quantity: i64,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct OrderBookData {
    pub account: String,
    pub orders: Vec<OrderRecord>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct OrderHistoryData {
    pub account: String,
    #[serde(rename = "orderHistories")]
    pub order_histories: Vec<OrderRecord>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct AuditOrderBookData {
    pub account: String,
    pub orders: Vec<AuditOrderRecord>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct OrderRecord {
    #[serde(rename = "uniqueID")]
    pub unique_id: Option<String>,
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(rename = "buySell")]
    pub buy_sell: String,
    pub price: DecimalNumber,
    pub quantity: i64,
    #[serde(rename = "filledQty")]
    pub filled_quantity: i64,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "marketID")]
    pub market_id: String,
    #[serde(rename = "inputTime")]
    pub input_time: String,
    #[serde(rename = "modifiedTime")]
    pub modified_time: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "cancelQty")]
    pub cancelled_quantity: i64,
    #[serde(rename = "avgPrice")]
    pub average_price: DecimalNumber,
    #[serde(rename = "isForcesell", alias = "isForceSell")]
    pub is_force_sell: Option<String>,
    #[serde(rename = "isShortsell", alias = "isShortSell")]
    pub is_short_sell: Option<String>,
    #[serde(rename = "rejectReason")]
    pub reject_reason: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct AuditOrderRecord {
    #[serde(flatten)]
    pub order: OrderRecord,
    #[serde(rename = "lastErrorEvent")]
    pub last_error_event: Option<Box<Self>>,
}
