#![cfg(test)]

use ssi_fc_trading::{ApiResponse, CashAccountBalance, OrderRecord, RawData};

#[test]
fn envelope_rejects_missing_message_when_response_is_decoded() {
    // Given
    let response = r#"{"status":200,"data":{}}"#;

    // When
    let result = serde_json::from_str::<ApiResponse<RawData>>(response);

    // Then
    assert!(result.is_err());
}

#[test]
fn cash_balance_rejects_missing_required_field_when_response_is_decoded() {
    // Given
    let response = serde_json::json!({
        "account": "123456",
        "cashBal": 1,
        "cashOnHold": 2,
        "secureAmount": 3,
        "withdrawable": 4,
        "receivingCashT1": 5,
        "receivingCashT2": 6,
        "matchedBuyVolume": 7,
        "matchedSellVolume": 8,
        "unMatchedBuyVolume": 9,
        "unMatchedSellVolume": 10,
        "paidCashT1": 11,
        "paidCashT2": 12,
        "cia": 13,
        "debt": 14,
        "purchasingPower": 15
    });

    // When
    let result = serde_json::from_value::<CashAccountBalance>(response);

    // Then
    assert!(result.is_err());
}

#[test]
fn nullable_order_fields_are_preserved_when_response_is_decoded() {
    // Given
    let response = serde_json::json!({
        "uniqueID": null,
        "orderID": "order-1",
        "buySell": "B",
        "price": 1000,
        "quantity": 10,
        "filledQty": 0,
        "orderStatus": "RS",
        "marketID": "VN",
        "inputTime": "1",
        "modifiedTime": "2",
        "instrumentID": "FPT",
        "orderType": "LO",
        "cancelQty": 0,
        "avgPrice": 0,
        "isForcesell": null,
        "isShortsell": null
    });

    // When
    let result = serde_json::from_value::<OrderRecord>(response);

    // Then
    assert!(result.is_ok());
}
