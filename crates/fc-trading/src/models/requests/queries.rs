use serde::Serialize;

use crate::DecimalNumber;

#[derive(Clone, Debug, Serialize)]
pub struct AccountRequest {
    pub account: String,
}

impl AccountRequest {
    #[must_use]
    pub fn new(account: impl Into<String>) -> Self {
        Self {
            account: account.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DerivativePositionRequest {
    pub account: String,
    #[serde(rename = "querySummary")]
    pub query_summary: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct MaxQuantityRequest {
    pub account: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    pub price: DecimalNumber,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrderHistoryRequest {
    pub account: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}

pub type StockAccountBalanceRequest = AccountRequest;
pub type DerivativeAccountBalanceRequest = AccountRequest;
pub type PpmmrAccountRequest = AccountRequest;
pub type StockPositionRequest = AccountRequest;
pub type OrderBookRequest = AccountRequest;
pub type AuditOrderBookRequest = AccountRequest;
pub type MaxBuyQuantityRequest = MaxQuantityRequest;
pub type MaxSellQuantityRequest = MaxQuantityRequest;
