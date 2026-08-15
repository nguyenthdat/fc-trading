use serde::Serialize;

use super::{AccountRequest, OrderHistoryRequest};

#[derive(Clone, Debug, Serialize)]
pub struct StockTransferRequest {
    pub account: String,
    #[serde(rename = "beneficiaryAccount")]
    pub beneficiary_account: String,
    #[serde(rename = "exchangeID")]
    pub exchange_id: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    pub quantity: i64,
    pub code: String,
}

pub type StockTransferableRequest = AccountRequest;
pub type StockTransferHistoryRequest = OrderHistoryRequest;
