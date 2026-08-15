use super::TradingClient;
use super::endpoints::{CREATE_ORS, ORS_DIVIDEND, ORS_EXERCISABLE_QUANTITY, ORS_HISTORY};
use crate::{AccountRequest, ApiResponse, OrderHistoryRequest, OrsRequest, RawData, Result};

impl TradingClient {
    pub async fn ors_dividend(&self, request: &AccountRequest) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(ORS_DIVIDEND, request).await
    }

    pub async fn ors_exercisable_quantity(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(ORS_EXERCISABLE_QUANTITY, request)
            .await
    }

    pub async fn ors_history(&self, request: &OrderHistoryRequest) -> Result<ApiResponse<RawData>> {
        self.authenticated_get(ORS_HISTORY, request).await
    }

    pub async fn create_ors(&self, request: &OrsRequest) -> Result<ApiResponse<RawData>> {
        self.signed_post(CREATE_ORS, request).await
    }
}
