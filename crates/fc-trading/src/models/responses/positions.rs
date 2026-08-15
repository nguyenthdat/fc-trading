use serde::Deserialize;

use crate::DecimalNumber;

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct StockPositionData {
    pub account: String,
    #[serde(rename = "totalMarketValue")]
    pub total_market_value: DecimalNumber,
    #[serde(rename = "stockPositions")]
    pub stock_positions: Vec<StockPosition>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct StockPosition {
    #[serde(rename = "marketID")]
    pub market_id: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    #[serde(rename = "onHand")]
    pub on_hand: i64,
    pub block: i64,
    pub bonus: i64,
    #[serde(rename = "buyT0")]
    pub buy_t0: i64,
    #[serde(rename = "buyT1")]
    pub buy_t1: i64,
    #[serde(rename = "buyT2")]
    pub buy_t2: i64,
    #[serde(rename = "sellT0")]
    pub sell_t0: i64,
    #[serde(rename = "sellT1")]
    pub sell_t1: i64,
    #[serde(rename = "sellT2")]
    pub sell_t2: i64,
    #[serde(rename = "avgPrice")]
    pub average_price: DecimalNumber,
    pub mortgage: i64,
    #[serde(rename = "sellableQty")]
    pub sellable_quantity: i64,
    #[serde(rename = "holdForTrade")]
    pub held_for_trade: i64,
    #[serde(rename = "marketPrice")]
    pub market_price: DecimalNumber,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct DerivativePositionData {
    pub account: String,
    #[serde(rename = "openPosition")]
    pub open_positions: Vec<DerivativePosition>,
    #[serde(rename = "closePosition")]
    pub closed_positions: Vec<DerivativePosition>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct DerivativePosition {
    #[serde(rename = "marketID")]
    pub market_id: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    #[serde(rename = "longQty")]
    pub long_quantity: i64,
    #[serde(rename = "shortQty")]
    pub short_quantity: i64,
    pub net: i64,
    #[serde(rename = "bidAvgPrice")]
    pub bid_average_price: DecimalNumber,
    #[serde(rename = "askAvgPrice")]
    pub ask_average_price: DecimalNumber,
    #[serde(rename = "tradePrice")]
    pub trade_price: DecimalNumber,
    #[serde(rename = "marketPrice")]
    pub market_price: DecimalNumber,
    #[serde(rename = "floatingPL")]
    pub floating_pl: DecimalNumber,
    #[serde(rename = "tradingPL")]
    pub trading_pl: DecimalNumber,
}
