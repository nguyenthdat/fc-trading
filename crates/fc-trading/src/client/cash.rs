use super::TradingClient;
use super::endpoints::{
    CASH_IN_ADVANCE_AMOUNT, CASH_IN_ADVANCE_ESTIMATE, CASH_IN_ADVANCE_HISTORY, CASH_TRANSFER,
    CASH_TRANSFER_HISTORY, CASH_TRANSFER_VSD, CREATE_CASH_IN_ADVANCE, UNSETTLED_SOLD_TRANSACTION,
};
use crate::{
    AccountRequest, ApiResponse, CashInAdvanceEstimateRequest, CashInAdvanceHistoryRequest,
    CashInAdvanceRequest, CashTransferHistoryRequest, CashTransferRequest, CashTransferVsdRequest,
    RawData, Result, UnsettledSoldTransactionRequest,
};

impl TradingClient {
    pub async fn cash_in_advance_amount(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(CASH_IN_ADVANCE_AMOUNT, request)
            .await
    }

    pub async fn unsettled_sold_transaction(
        &self,
        request: &UnsettledSoldTransactionRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(UNSETTLED_SOLD_TRANSACTION, request)
            .await
    }

    pub async fn cash_transfer_history(
        &self,
        request: &CashTransferHistoryRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(CASH_TRANSFER_HISTORY, request).await
    }

    pub async fn cash_in_advance_history(
        &self,
        request: &CashInAdvanceHistoryRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(CASH_IN_ADVANCE_HISTORY, request)
            .await
    }

    pub async fn cash_in_advance_estimate(
        &self,
        request: &CashInAdvanceEstimateRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(CASH_IN_ADVANCE_ESTIMATE, request)
            .await
    }

    pub async fn cash_transfer_vsd(
        &self,
        request: &CashTransferVsdRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.signed_post(CASH_TRANSFER_VSD, request).await
    }

    pub async fn cash_transfer(
        &self,
        request: &CashTransferRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.signed_post(CASH_TRANSFER, request).await
    }

    pub async fn create_cash_in_advance(
        &self,
        request: &CashInAdvanceRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.signed_post(CREATE_CASH_IN_ADVANCE, request).await
    }
}
