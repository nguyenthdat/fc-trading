use serde::Serialize;

use crate::DecimalNumber;

use super::AccountRequest;

#[derive(Clone, Debug, Serialize)]
pub struct UnsettledSoldTransactionRequest {
    pub account: String,
    #[serde(rename = "settleDate")]
    pub settle_date: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CashTransferHistoryRequest {
    pub account: String,
    #[serde(rename = "fromDate")]
    pub from_date: String,
    #[serde(rename = "toDate")]
    pub to_date: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CashInAdvanceHistoryRequest {
    pub account: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CashInAdvanceEstimateRequest {
    pub account: String,
    #[serde(rename = "ciaAmount")]
    pub cia_amount: DecimalNumber,
    #[serde(rename = "receiveAmount")]
    pub receive_amount: DecimalNumber,
}

#[derive(Clone, Debug, Serialize)]
pub struct CashTransferVsdRequest {
    pub account: String,
    pub amount: i64,
    #[serde(rename = "type")]
    pub transfer_type: String,
    pub remark: String,
    pub code: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CashTransferRequest {
    pub account: String,
    #[serde(rename = "beneficiaryAccount")]
    pub beneficiary_account: String,
    pub amount: i64,
    pub remark: String,
    pub code: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CashInAdvanceRequest {
    pub account: String,
    #[serde(rename = "ciaAmount")]
    pub cia_amount: DecimalNumber,
    #[serde(rename = "receiveAmount")]
    pub receive_amount: DecimalNumber,
    pub code: String,
}

pub type CashInAdvanceAmountRequest = AccountRequest;
