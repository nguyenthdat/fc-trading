#![cfg(test)]

use std::str::FromStr as _;

use serde_json::Value;
use ssi_fc_trading::{
    CancelOrderRequest, DecimalNumber, ModifyOrderRequest, NewOrderRequest, TwoFactorType,
};

#[test]
fn new_order_serializes_python_wire_shape_when_defaults_are_present() {
    // Given
    let request = NewOrderRequest {
        account: "123456".to_owned(),
        request_id: "42".to_owned(),
        instrument_id: "FPT".to_owned(),
        market: "VN".to_owned(),
        buy_sell: "B".to_owned(),
        order_type: "LO".to_owned(),
        price: DecimalNumber::from_str("12345.5").expect("valid decimal fixture"),
        quantity: 10,
        stop_order: false,
        stop_price: DecimalNumber::ZERO,
        stop_type: String::new(),
        stop_step: DecimalNumber::ZERO,
        loss_step: DecimalNumber::ZERO,
        profit_step: DecimalNumber::ZERO,
        channel_id: "TA".to_owned(),
        code: String::new(),
        device_id: "AA:BB:CC:DD:EE:FF".to_owned(),
        user_agent: "ssi-fc-trading-test".to_owned(),
        modifiable: None,
    };

    // When
    let serialized = serde_json::to_value(&request).expect("request serializes");

    // Then
    assert_eq!(serialized, fixture("new_order"));
}

#[test]
fn modify_order_serializes_python_wire_shape_when_request_is_created() {
    // Given
    let request = ModifyOrderRequest {
        account: "123456".to_owned(),
        request_id: "43".to_owned(),
        order_id: "order-1".to_owned(),
        market_id: "VN".to_owned(),
        instrument_id: "FPT".to_owned(),
        price: DecimalNumber::from_str("12346").expect("valid decimal fixture"),
        quantity: 9,
        buy_sell: "B".to_owned(),
        order_type: "LO".to_owned(),
        code: String::new(),
        device_id: "AA:BB:CC:DD:EE:FF".to_owned(),
        user_agent: "ssi-fc-trading-test".to_owned(),
    };

    // When
    let serialized = serde_json::to_value(&request).expect("request serializes");

    // Then
    assert_eq!(serialized, fixture("modify_order"));
}

#[test]
fn cancel_order_serializes_python_wire_shape_when_request_is_created() {
    // Given
    let request = CancelOrderRequest {
        account: "123456".to_owned(),
        request_id: "44".to_owned(),
        order_id: "order-1".to_owned(),
        market_id: "VN".to_owned(),
        instrument_id: "FPT".to_owned(),
        buy_sell: "B".to_owned(),
        code: String::new(),
        device_id: "AA:BB:CC:DD:EE:FF".to_owned(),
        user_agent: "ssi-fc-trading-test".to_owned(),
    };

    // When
    let serialized = serde_json::to_value(&request).expect("request serializes");

    // Then
    assert_eq!(serialized, fixture("cancel_order"));
}

#[test]
fn two_factor_type_serializes_numeric_value_when_pin_is_selected() {
    // Given
    let factor = TwoFactorType::Pin;

    // When
    let serialized = serde_json::to_string(&factor).expect("factor serializes");

    // Then
    assert_eq!(serialized, "0");
}

fn fixture(name: &str) -> Value {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/core_requests.json"))
        .expect("request fixtures are valid JSON");
    fixtures.get(name).cloned().expect("request fixture exists")
}
