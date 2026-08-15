#![cfg(test)]

use std::str::FromStr as _;

use serde::Serialize;
use serde_json::Value;
use ssi_fc_trading::{
    CashInAdvanceRequest, CashTransferRequest, CashTransferVsdRequest, DecimalNumber, OrsRequest,
    StockTransferRequest,
};

#[test]
fn cash_transfer_vsd_serializes_contract_fixture_when_request_is_created() {
    let request = CashTransferVsdRequest {
        account: "123456".to_owned(),
        amount: 1000,
        transfer_type: "D".to_owned(),
        remark: "fixture".to_owned(),
        code: String::new(),
    };
    assert_fixture("cash_transfer_vsd", &request);
}

#[test]
fn cash_transfer_serializes_contract_fixture_when_request_is_created() {
    let request = CashTransferRequest {
        account: "123456".to_owned(),
        beneficiary_account: "654321".to_owned(),
        amount: 1000,
        remark: "fixture".to_owned(),
        code: String::new(),
    };
    assert_fixture("cash_transfer", &request);
}

#[test]
fn cash_advance_serializes_contract_fixture_when_request_is_created() {
    let request = CashInAdvanceRequest {
        account: "123456".to_owned(),
        cia_amount: DecimalNumber::from_str("1000").expect("valid decimal fixture"),
        receive_amount: DecimalNumber::from_str("900").expect("valid decimal fixture"),
        code: String::new(),
    };
    assert_fixture("cash_in_advance", &request);
}

#[test]
fn ors_serializes_contract_fixture_when_request_is_created() {
    let request = OrsRequest {
        account: "123456".to_owned(),
        instrument_id: "FPT".to_owned(),
        entitlement_id: "entitlement-1".to_owned(),
        quantity: 10,
        amount: DecimalNumber::from_str("500").expect("valid decimal fixture"),
        code: String::new(),
    };
    assert_fixture("ors", &request);
}

#[test]
fn stock_transfer_serializes_contract_fixture_when_request_is_created() {
    let request = StockTransferRequest {
        account: "123456".to_owned(),
        beneficiary_account: "654321".to_owned(),
        exchange_id: "HOSE".to_owned(),
        instrument_id: "FPT".to_owned(),
        quantity: 10,
        code: String::new(),
    };
    assert_fixture("stock_transfer", &request);
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
