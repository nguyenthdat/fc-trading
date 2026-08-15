#![cfg(test)]

use serde::de::DeserializeOwned;
use serde_json::Value;
use ssi_fc_trading::{
    ApiResponse, CashAccountBalance, DerivativeAccountBalance, DerivativePositionData,
    PpmmrAccountBalance, StockPositionData,
};

const FIXTURES: &str = include_str!("fixtures/core_account_responses.json");

#[test]
fn cash_account_balance_decodes_all_required_fields_from_contract_fixture() {
    // When
    let response: ApiResponse<CashAccountBalance> = decode("cash_account_balance");

    // Then
    let data = response.data.expect("cash balance data exists");
    assert_eq!(data.account, "123456");
    assert_eq!(data.cash_balance.as_decimal().to_string(), "1001");
    assert_eq!(data.total_assets.as_decimal().to_string(), "1016");
}

#[test]
fn derivative_account_balance_decodes_nested_contract_fixture() {
    // When
    let response: ApiResponse<DerivativeAccountBalance> = decode("derivative_account_balance");

    // Then
    let data = response.data.expect("derivative balance data exists");
    assert_eq!(data.internal_assets.cash.as_decimal().to_string(), "2101");
    assert_eq!(
        data.exchange_margin.margin_call.as_decimal().to_string(),
        "2406"
    );
}

#[test]
fn ppmmr_account_balance_decodes_compatibility_alias_from_contract_fixture() {
    // When
    let response: ApiResponse<PpmmrAccountBalance> = decode("ppmmr_account_balance");

    // Then
    let data = response.data.expect("PPMMR balance data exists");
    assert_eq!(data.accrued_interest.as_decimal().to_string(), "3018");
    assert_eq!(data.total_equity.as_decimal().to_string(), "3035");
}

#[test]
fn stock_position_decodes_required_collection_from_contract_fixture() {
    // When
    let response: ApiResponse<StockPositionData> = decode("stock_position");

    // Then
    let data = response.data.expect("stock position data exists");
    assert_eq!(data.stock_positions.len(), 1);
    assert_eq!(data.stock_positions[0].instrument_id, "FPT");
    assert_eq!(
        data.stock_positions[0]
            .average_price
            .as_decimal()
            .to_string(),
        "12345.5"
    );
}

#[test]
fn derivative_position_decodes_required_open_and_closed_collections() {
    // When
    let response: ApiResponse<DerivativePositionData> = decode("derivative_position");

    // Then
    let data = response.data.expect("derivative position data exists");
    assert_eq!(data.open_positions.len(), 1);
    assert!(data.closed_positions.is_empty());
    assert_eq!(data.open_positions[0].instrument_id, "VN30F2601");
}

fn decode<T>(name: &str) -> ApiResponse<T>
where
    T: DeserializeOwned,
{
    let fixtures: Value = serde_json::from_str(FIXTURES).expect("account fixtures are valid JSON");
    serde_json::from_value(fixtures[name].clone()).expect("account fixture matches contract")
}
