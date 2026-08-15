#![cfg(test)]

use serde::de::DeserializeOwned;
use serde_json::Value;
use ssi_fc_trading::{
    ApiResponse, AuditOrderBookData, CancelOrderData, MaxBuyQuantityData, MaxSellQuantityData,
    ModifyOrderData, NewOrderData, OrderBookData, OrderHistoryData, RawData,
};

const FIXTURES: &str = include_str!("fixtures/core_order_responses.json");

#[test]
fn raw_response_preserves_endpoint_specific_data_from_contract_fixture() {
    let response: ApiResponse<RawData> = decode("raw");
    assert_eq!(response.data.expect("raw data exists")["quota"], 10);
}

#[test]
fn max_buy_quantity_decodes_contract_fixture() {
    let response: ApiResponse<MaxBuyQuantityData> = decode("max_buy_quantity");
    let data = response.data.expect("max-buy data exists");
    assert_eq!(data.max_buy_quantity, 10);
    assert_eq!(data.purchasing_power.as_decimal().to_string(), "100000");
}

#[test]
fn max_sell_quantity_decodes_contract_fixture() {
    let response: ApiResponse<MaxSellQuantityData> = decode("max_sell_quantity");
    assert_eq!(
        response
            .data
            .expect("max-sell data exists")
            .max_sell_quantity,
        9
    );
}

#[test]
fn new_order_echo_decodes_contract_fixture() {
    let response: ApiResponse<NewOrderData> = decode("new_order");
    let data = response.data.expect("new-order data exists");
    assert_eq!(data.request_id, "42");
    assert_eq!(data.request_data.instrument_id, "FPT");
}

#[test]
fn modify_order_echo_decodes_pascal_case_alias_fixture() {
    let response: ApiResponse<ModifyOrderData> = decode("modify_order");
    let data = response.data.expect("modify-order data exists");
    assert_eq!(data.request_id, "43");
    assert_eq!(data.request_data.order_id, "order-1");
}

#[test]
fn cancel_order_echo_decodes_pascal_case_alias_fixture() {
    let response: ApiResponse<CancelOrderData> = decode("cancel_order");
    let data = response.data.expect("cancel-order data exists");
    assert_eq!(data.request_id, "44");
    assert_eq!(data.request_data.request_id, "44");
}

#[test]
fn order_history_preserves_documented_nullable_fields() {
    let response: ApiResponse<OrderHistoryData> = decode("order_history");
    let data = response.data.expect("order-history data exists");
    assert_eq!(data.order_histories.len(), 1);
    assert_eq!(data.order_histories[0].unique_id, None);
    assert_eq!(data.order_histories[0].reject_reason, None);
}

#[test]
fn order_book_decodes_envelope_and_field_aliases() {
    let response: ApiResponse<OrderBookData> = decode("order_book");
    let data = response.data.expect("order-book data exists");
    assert_eq!(response.message, "ok");
    assert_eq!(data.orders[0].is_force_sell.as_deref(), Some("F"));
    assert_eq!(data.orders[0].is_short_sell.as_deref(), Some("F"));
}

#[test]
fn audit_order_book_decodes_nullable_recursive_error_event() {
    let response: ApiResponse<AuditOrderBookData> = decode("audit_order_book");
    let data = response.data.expect("audit-order data exists");
    assert!(data.orders[0].last_error_event.is_none());
}

fn decode<T>(name: &str) -> ApiResponse<T>
where
    T: DeserializeOwned,
{
    let fixtures: Value = serde_json::from_str(FIXTURES).expect("order fixtures are valid JSON");
    serde_json::from_value(fixtures[name].clone()).expect("order fixture matches contract")
}
