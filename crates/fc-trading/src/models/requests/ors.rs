use serde::Serialize;

use crate::DecimalNumber;

use super::{AccountRequest, OrderHistoryRequest};

#[derive(Clone, Debug, Serialize)]
pub struct OrsRequest {
    pub account: String,
    #[serde(rename = "instrumentID")]
    pub instrument_id: String,
    #[serde(rename = "entitlementID")]
    pub entitlement_id: String,
    pub quantity: i64,
    pub amount: DecimalNumber,
    pub code: String,
}

pub type OrsDividendRequest = AccountRequest;
pub type OrsExercisableQuantityRequest = AccountRequest;
pub type OrsHistoryRequest = OrderHistoryRequest;
