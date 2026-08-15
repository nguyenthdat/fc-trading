use super::TradingClient;
use super::endpoints::{CREATE_STOCK_TRANSFER, STOCK_TRANSFER_HISTORY, STOCK_TRANSFERABLE};
use crate::{
    AccountRequest, ApiResponse, OrderHistoryRequest, RawData, Result, StockTransferRequest,
};

impl TradingClient {
    pub async fn stock_transferable(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(STOCK_TRANSFERABLE, request).await
    }

    pub async fn stock_transfer_history(
        &self,
        request: &OrderHistoryRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(STOCK_TRANSFER_HISTORY, request)
            .await
    }

    pub async fn create_stock_transfer(
        &self,
        request: &StockTransferRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.signed_post(CREATE_STOCK_TRANSFER, request).await
    }
}
