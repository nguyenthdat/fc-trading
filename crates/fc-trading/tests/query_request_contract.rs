#![cfg(test)]

use std::str::FromStr as _;

use serde::Serialize;
use serde_json::Value;
use ssi_fc_trading::{
    AccountRequest, CashInAdvanceEstimateRequest, CashInAdvanceHistoryRequest,
    CashTransferHistoryRequest, DecimalNumber, DerivativePositionRequest, MaxQuantityRequest,
    OrderHistoryRequest, UnsettledSoldTransactionRequest,
};

#[test]
fn account_query_serializes_contract_fixture_when_request_is_created() {
    assert_fixture("account", &AccountRequest::new("123456"));
}

#[test]
fn derivative_position_query_serializes_contract_fixture_when_request_is_created() {
    let request = DerivativePositionRequest {
        account: "123456".to_owned(),
        query_summary: true,
    };
    assert_fixture("derivative_position", &request);
}

#[test]
fn max_quantity_query_serializes_contract_fixture_when_request_is_created() {
    let request = MaxQuantityRequest {
        account: "123456".to_owned(),
        instrument_id: "FPT".to_owned(),
        price: DecimalNumber::from_str("12345.5").expect("valid decimal fixture"),
    };
    assert_fixture("max_quantity", &request);
}

#[test]
fn order_history_query_serializes_contract_fixture_when_request_is_created() {
    let request = OrderHistoryRequest {
        account: "123456".to_owned(),
        start_date: "01/01/2026".to_owned(),
        end_date: "02/01/2026".to_owned(),
    };
    assert_fixture("order_history", &request);
}

#[test]
fn unsettled_transaction_query_serializes_contract_fixture_when_request_is_created() {
    let request = UnsettledSoldTransactionRequest {
        account: "123456".to_owned(),
        settle_date: "02/01/2026".to_owned(),
    };
    assert_fixture("unsettled_sold_transaction", &request);
}

#[test]
fn cash_transfer_history_query_serializes_contract_fixture_when_request_is_created() {
    let request = CashTransferHistoryRequest {
        account: "123456".to_owned(),
        from_date: "01/01/2026".to_owned(),
        to_date: "02/01/2026".to_owned(),
    };
    assert_fixture("cash_transfer_history", &request);
}

#[test]
fn cash_advance_history_query_serializes_contract_fixture_when_request_is_created() {
    let request = CashInAdvanceHistoryRequest {
        account: "123456".to_owned(),
        start_date: "01/01/2026".to_owned(),
        end_date: "02/01/2026".to_owned(),
    };
    assert_fixture("cash_in_advance_history", &request);
}

#[test]
fn cash_advance_estimate_query_serializes_contract_fixture_when_request_is_created() {
    let request = CashInAdvanceEstimateRequest {
        account: "123456".to_owned(),
        cia_amount: DecimalNumber::from_str("1000").expect("valid decimal fixture"),
        receive_amount: DecimalNumber::from_str("900").expect("valid decimal fixture"),
    };
    assert_fixture("cash_in_advance_estimate", &request);
}

fn assert_fixture<T>(name: &str, request: &T)
where
    T: Serialize,
{
    // When
    let serialized = serde_json::to_value(request).expect("request serializes");

    // Then
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/core_requests.json"))
        .expect("request fixtures are valid JSON");
    assert_eq!(serialized, fixtures[name]);
}
