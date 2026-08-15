use super::TradingClient;
use super::endpoints::{
    AUDIT_ORDER_BOOK, CANCEL_ORDER, DERIVATIVE_ACCOUNT_BALANCE, DERIVATIVE_CANCEL_ORDER,
    DERIVATIVE_MODIFY_ORDER, DERIVATIVE_NEW_ORDER, DERIVATIVE_POSITION, MAX_BUY_QUANTITY,
    MAX_SELL_QUANTITY, MODIFY_ORDER, NEW_ORDER, ORDER_BOOK, ORDER_HISTORY, PPMMR_ACCOUNT,
    RATE_LIMIT, STOCK_ACCOUNT_BALANCE, STOCK_POSITION,
};
use crate::{
    AccountRequest, ApiResponse, AuditOrderBookData, CancelOrderData, CancelOrderRequest,
    CashAccountBalance, DerivativeAccountBalance, DerivativePositionData,
    DerivativePositionRequest, MaxBuyQuantityData, MaxQuantityRequest, MaxSellQuantityData,
    ModifyOrderData, ModifyOrderRequest, NewOrderData, NewOrderRequest, OrderBookData,
    OrderHistoryData, OrderHistoryRequest, PpmmrAccountBalance, RawData, Result, StockPositionData,
};

impl TradingClient {
    pub async fn new_order(&self, request: &NewOrderRequest) -> Result<ApiResponse<NewOrderData>> {
        self.signed_post(NEW_ORDER, request).await
    }

    pub async fn derivative_new_order(
        &self,
        request: &NewOrderRequest,
    ) -> Result<ApiResponse<NewOrderData>> {
        self.signed_post(DERIVATIVE_NEW_ORDER, request).await
    }

    pub async fn modify_order(
        &self,
        request: &ModifyOrderRequest,
    ) -> Result<ApiResponse<ModifyOrderData>> {
        self.signed_post(MODIFY_ORDER, request).await
    }

    pub async fn derivative_modify_order(
        &self,
        request: &ModifyOrderRequest,
    ) -> Result<ApiResponse<ModifyOrderData>> {
        self.signed_post(DERIVATIVE_MODIFY_ORDER, request).await
    }

    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> Result<ApiResponse<CancelOrderData>> {
        self.signed_post(CANCEL_ORDER, request).await
    }

    pub async fn derivative_cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> Result<ApiResponse<CancelOrderData>> {
        self.signed_post(DERIVATIVE_CANCEL_ORDER, request).await
    }

    pub async fn stock_account_balance(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<CashAccountBalance>> {
        self.authenticated_get(STOCK_ACCOUNT_BALANCE, request).await
    }

    pub async fn derivative_account_balance(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<DerivativeAccountBalance>> {
        self.authenticated_get(DERIVATIVE_ACCOUNT_BALANCE, request)
            .await
    }

    pub async fn ppmmr_account(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<PpmmrAccountBalance>> {
        self.authenticated_get(PPMMR_ACCOUNT, request).await
    }

    pub async fn stock_position(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<StockPositionData>> {
        self.authenticated_get(STOCK_POSITION, request).await
    }

    pub async fn derivative_position(
        &self,
        request: &DerivativePositionRequest,
    ) -> Result<ApiResponse<DerivativePositionData>> {
        self.authenticated_get(DERIVATIVE_POSITION, request).await
    }

    pub async fn max_buy_quantity(
        &self,
        request: &MaxQuantityRequest,
    ) -> Result<ApiResponse<MaxBuyQuantityData>> {
        self.authenticated_get(MAX_BUY_QUANTITY, request).await
    }

    pub async fn max_sell_quantity(
        &self,
        request: &MaxQuantityRequest,
    ) -> Result<ApiResponse<MaxSellQuantityData>> {
        self.authenticated_get(MAX_SELL_QUANTITY, request).await
    }

    pub async fn order_history(
        &self,
        request: &OrderHistoryRequest,
    ) -> Result<ApiResponse<OrderHistoryData>> {
        self.authenticated_get(ORDER_HISTORY, request).await
    }

    pub async fn order_book(&self, request: &AccountRequest) -> Result<ApiResponse<OrderBookData>> {
        self.authenticated_get(ORDER_BOOK, request).await
    }

    pub async fn audit_order_book(
        &self,
        request: &AccountRequest,
    ) -> Result<ApiResponse<AuditOrderBookData>> {
        self.authenticated_get(AUDIT_ORDER_BOOK, request).await
    }

    pub async fn rate_limit(&self) -> Result<ApiResponse<RawData>> {
        self.authenticated_get_without_query(RATE_LIMIT).await
    }
}
