# ssi-fc-trading

Async Rust client cho SSI FastConnect Trading, hỗ trợ xác thực, giao dịch, truy vấn tài khoản và streaming.

## Cài đặt

```bash
cargo add ssi-fc-trading secrecy
cargo add tokio --features macros,rt-multi-thread
```

## Ví dụ

```rust
use secrecy::SecretString;
use ssi_fc_trading::{
    AccountRequest, ClientConfig, Credentials, TradingClient, TwoFactorType,
};

#[tokio::main]
async fn main() -> ssi_fc_trading::Result<()> {
    let credentials = Credentials::from_base64_xml(
        "consumer-id",
        SecretString::from("consumer-secret"),
        SecretString::from("base64-xml-private-key"),
    )?;
    let client = TradingClient::new(
        ClientConfig::production()?,
        credentials,
        TwoFactorType::Pin,
    )?;

    let response = client
        .stock_account_balance(&AccountRequest::new("YOUR_ACCOUNT"))
        .await?;
    println!("{}", response.message);
    Ok(())
}
```

Xem [README của project](https://github.com/nguyenthdat/fc-trading#readme) để biết cấu hình, CLI và quy trình phát hành.

## License

MIT
