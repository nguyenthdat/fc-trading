use serde::Deserialize;

use crate::DecimalNumber;

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct OrderRequestData<T> {
    #[serde(rename = "requestID", alias = "RequestID")]
    pub request_id: String,
    #[serde(rename = "requestData", alias = "RequestData")]
    pub request_data: T,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct NewOrderEcho {
    #[serde(rename = "instrumentID", alias = "InstrumentID")]
    pub instrument_id: String,
    pub market: String,
    #[serde(rename = "buySell", alias = "BuySell")]
    pub buy_sell: String,
    #[serde(rename = "orderType", alias = "OrderType")]
    pub order_type: String,
    #[serde(rename = "channelID", alias = "ChannelID")]
    pub channel_id: String,
    pub price: DecimalNumber,
    pub quantity: i64,
    pub account: String,
    #[serde(rename = "stopOrder", alias = "StopOrder")]
    pub stop_order: bool,
    #[serde(rename = "stopPrice", alias = "StopPrice")]
    pub stop_price: DecimalNumber,
    #[serde(rename = "stopType", alias = "StopType")]
    pub stop_type: String,
    #[serde(rename = "stopStep", alias = "StopStep")]
    pub stop_step: DecimalNumber,
    #[serde(rename = "lossStep", alias = "LossStep")]
    pub loss_step: DecimalNumber,
    #[serde(rename = "profitStep", alias = "ProfitStep")]
    pub profit_step: DecimalNumber,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct CancelOrderEcho {
    #[serde(rename = "orderID", alias = "OrderID")]
    pub order_id: String,
    pub account: String,
    #[serde(rename = "marketID", alias = "MarketID")]
    pub market_id: String,
    #[serde(rename = "instrumentID", alias = "InstrumentID")]
    pub instrument_id: String,
    #[serde(rename = "buySell", alias = "BuySell")]
    pub buy_sell: String,
    #[serde(rename = "requestID", alias = "RequestID")]
    pub request_id: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct ModifyOrderEcho {
    #[serde(rename = "orderID", alias = "OrderID")]
    pub order_id: String,
    #[serde(alias = "Price")]
    pub price: DecimalNumber,
    #[serde(alias = "Quantity")]
    pub quantity: i64,
    #[serde(alias = "Account")]
    pub account: String,
    #[serde(rename = "instrumentID", alias = "InstrumentID")]
    pub instrument_id: String,
    #[serde(rename = "marketID", alias = "MarketID")]
    pub market_id: String,
    #[serde(rename = "buySell", alias = "BuySell")]
    pub buy_sell: String,
    #[serde(rename = "orderType", alias = "OrderType")]
    pub order_type: String,
}

pub type NewOrderData = OrderRequestData<NewOrderEcho>;
pub type CancelOrderData = OrderRequestData<CancelOrderEcho>;
pub type ModifyOrderData = OrderRequestData<ModifyOrderEcho>;
