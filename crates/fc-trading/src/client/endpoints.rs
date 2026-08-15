#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum HttpMethod {
    Get,
    Post,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AuthClass {
    None,
    Read,
    Write,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Endpoint {
    #[cfg(test)]
    pub(super) name: &'static str,
    #[cfg(test)]
    pub(super) method: HttpMethod,
    pub(super) path: &'static str,
    #[cfg(test)]
    pub(super) auth: AuthClass,
}

macro_rules! endpoint_table {
    ($($constant:ident => ($name:literal, $method:ident, $path:literal, $auth:ident);)+) => {
        $(pub(super) const $constant: Endpoint = Endpoint {
            #[cfg(test)]
            name: $name,
            #[cfg(test)]
            method: HttpMethod::$method,
            path: $path,
            #[cfg(test)]
            auth: AuthClass::$auth,
        };)+

        #[cfg(test)]
        pub(super) const ALL: &[Endpoint] = &[$($constant),+];
    };
}

endpoint_table! {
    REQUEST_OTP => ("request_otp", Post, "/api/v2/Trading/GetOTP", None);
    NEW_ORDER => ("new_order", Post, "/api/v2/Trading/NewOrder", Write);
    MODIFY_ORDER => ("modify_order", Post, "/api/v2/Trading/ModifyOrder", Write);
    CANCEL_ORDER => ("cancel_order", Post, "/api/v2/Trading/CancelOrder", Write);
    DERIVATIVE_NEW_ORDER => ("derivative_new_order", Post, "/api/v2/Trading/derNewOrder", Write);
    DERIVATIVE_MODIFY_ORDER => ("derivative_modify_order", Post, "/api/v2/Trading/derModifyOrder", Write);
    DERIVATIVE_CANCEL_ORDER => ("derivative_cancel_order", Post, "/api/v2/Trading/derCancelOrder", Write);
    STOCK_ACCOUNT_BALANCE => ("stock_account_balance", Get, "/api/v2/Trading/cashAcctBal", Read);
    DERIVATIVE_ACCOUNT_BALANCE => ("derivative_account_balance", Get, "/api/v2/Trading/derivAcctBal", Read);
    PPMMR_ACCOUNT => ("ppmmr_account", Get, "/api/v2/Trading/ppmmraccount", Read);
    STOCK_POSITION => ("stock_position", Get, "/api/v2/Trading/stockPosition", Read);
    DERIVATIVE_POSITION => ("derivative_position", Get, "/api/v2/Trading/derivPosition", Read);
    MAX_BUY_QUANTITY => ("max_buy_quantity", Get, "/api/v2/Trading/maxBuyQty", Read);
    MAX_SELL_QUANTITY => ("max_sell_quantity", Get, "/api/v2/Trading/maxSellQty", Read);
    ORDER_HISTORY => ("order_history", Get, "/api/v2/Trading/orderHistory", Read);
    ORDER_BOOK => ("order_book", Get, "/api/v2/Trading/orderBook", Read);
    AUDIT_ORDER_BOOK => ("audit_order_book", Get, "/api/v2/Trading/auditOrderBook", Read);
    RATE_LIMIT => ("rate_limit", Get, "/api/v2/Trading/rateLimit", Read);
    CASH_IN_ADVANCE_AMOUNT => ("cash_in_advance_amount", Get, "/api/v2/cash/cashInAdvanceAmount", Read);
    UNSETTLED_SOLD_TRANSACTION => ("unsettled_sold_transaction", Get, "/api/v2/cash/unsettleSoldTransaction", Read);
    CASH_TRANSFER_HISTORY => ("cash_transfer_history", Get, "/api/v2/cash/transferHistories", Read);
    CASH_IN_ADVANCE_HISTORY => ("cash_in_advance_history", Get, "/api/v2/cash/cashInAdvanceHistories", Read);
    CASH_IN_ADVANCE_ESTIMATE => ("cash_in_advance_estimate", Get, "/api/v2/cash/estCashInAdvanceFee", Read);
    CASH_TRANSFER_VSD => ("cash_transfer_vsd", Post, "/api/v2/cash/vsdCashDW", Write);
    CASH_TRANSFER => ("cash_transfer", Post, "/api/v2/cash/transferInternal", Write);
    CREATE_CASH_IN_ADVANCE => ("create_cash_in_advance", Post, "/api/v2/cash/createCashInAdvance", Write);
    ORS_DIVIDEND => ("ors_dividend", Get, "/api/v2/ors/dividend", Read);
    ORS_EXERCISABLE_QUANTITY => ("ors_exercisable_quantity", Get, "/api/v2/ors/exercisableQuantity", Read);
    ORS_HISTORY => ("ors_history", Get, "/api/v2/ors/histories", Read);
    CREATE_ORS => ("create_ors", Post, "/api/v2/ors/create", Write);
    STOCK_TRANSFERABLE => ("stock_transferable", Get, "/api/v2/stock/transferable", Read);
    STOCK_TRANSFER_HISTORY => ("stock_transfer_history", Get, "/api/v2/stock/transferHistories", Read);
    CREATE_STOCK_TRANSFER => ("create_stock_transfer", Post, "/api/v2/stock/transfer", Write);
}
