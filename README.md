# ssi-fc-trading

Thư viện Rust thuần cho SSI FastConnect Trading, được đối chiếu từ:

- `ssi-fctrading` Python 2.5.3 cho route và request wire contract, đã được lưu lại thành offline fixtures.
- `SSI.FCTrading.Client` .NET cho response model cốt lõi và hành vi ký request, đã được lưu lại thành offline fixtures.
- SSI FastConnect Trading specification v2.4 cho enum, field và streaming event.

## Thành phần

- `crates/fc-trading`: package async `ssi-fc-trading`, import trong Rust bằng `ssi_fc_trading`.
- `crates/fc-trading-cli`: CLI chẩn đoán an toàn, không có lệnh đặt/cancel/modify order.

Thư viện có 33 catalogued operations khớp endpoint catalog của Python 2.5.3, cộng route `/api/v2/Trading/AccessToken` được dùng riêng cho read/write authentication. Tổng cộng có 34 HTTP paths khác nhau:

- Authentication và OTP.
- Order chứng khoán/phái sinh.
- Balance, position, order history, order book và rate limit.
- Cash transfer/cash in advance.
- ORS.
- Stock transfer.
- Classic ASP.NET SignalR 1.3 qua `BroadcastHubV2`.

## Phạm vi tương thích

Parity với Python 2.5.3/.NET và literal conformance với specification v2.4 là hai tiêu chí riêng:

- Route, request casing, token separation, signature và SignalR transport bám theo hai SDK tham chiếu.
- Specification v2.4 chỉ mô tả 19 REST operations và có các điểm mâu thuẫn với SDK hiện hành, gồm `/verifyCode` so với `/AccessToken`, `maxSellQty.price`, casing của `orderBook`, `X-Singature`, status âm/dương và kiểu `stopOrder`.
- `verify_code` chủ ý dùng `/api/v2/Trading/AccessToken` giống Python/.NET; đây không phải literal implementation của endpoint `/api/v2/Trading/verifyCode` trong v2.4.
- Core success responses được decode strict: field bắt buộc bị thiếu sẽ trả lỗi. `data` và các field được tài liệu xác định nullable vẫn dùng `Option`.
- Cash, ORS, stock-transfer và rate-limit response chưa có schema ổn định trong SDK tham chiếu nên vẫn trả `ApiResponse<serde_json::Value>`.

## Cấu hình

Copy `.env.example` thành `.env` và điền credential. `.env` đã được ignore bởi Git.

Các biến bắt buộc:

```text
SSI_FCTRADING_CONSUMER_ID
SSI_FCTRADING_CONSUMER_SECRET
SSI_FCTRADING_PRIVATE_KEY
```

`SSI_FCTRADING_PUBLIC_KEY` được giữ trong `.env.example` để tương thích bộ credential hiện tại, nhưng giao thức client không sử dụng giá trị này.

## Sử dụng thư viện

```rust,no_run
use ssi_fc_trading::{
    AccountRequest, ClientConfig, Credentials, TradingClient, TwoFactorType,
};
use secrecy::SecretString;

# async fn run() -> ssi_fc_trading::Result<()> {
let config = ClientConfig::production()?;
let credentials = Credentials::from_base64_xml(
    "consumer-id",
    SecretString::from("consumer-secret"),
    SecretString::from("base64-xml-private-key"),
)?;
let client = TradingClient::new(config, credentials, TwoFactorType::Pin)?;

let balance = client
    .stock_account_balance(&AccountRequest::new("account"))
    .await?;
assert_eq!(balance.status, 200);
# Ok(())
# }
```

GET sử dụng read token. Mọi POST thay đổi trạng thái cần gọi `verify_code` trước:

```rust,no_run
# use ssi_fc_trading::{ClientConfig, Credentials, TradingClient, TwoFactorType};
# async fn run(client: TradingClient) -> ssi_fc_trading::Result<()> {
client.verify_code("otp-or-pin").await?;
# Ok(())
# }
```

Write token không tự refresh và request POST không tự retry. Điều này tránh lặp lệnh giao dịch khi kết nối gặp lỗi.

## CLI an toàn

```bash
cargo run -p fctrading-cli -- --help
cargo run -p fctrading-cli -- doctor
cargo run -p fctrading-cli -- request-otp
cargo run -p fctrading-cli -- verify-write
cargo run -p fctrading-cli -- stream --notify-id -1 --seconds 30
```

`doctor` chỉ lấy read token và gọi `rateLimit`. `request-otp` và `verify-write` chỉ chạy khi được gọi rõ ràng. CLI không cung cấp thao tác đặt lệnh, chuyển tiền hay thực hiện quyền.

## Bảo mật

- Request body được serialize đúng một lần; cùng byte buffer được ký và gửi.
- Chữ ký là RSA PKCS#1 v1.5 SHA-256, mã hóa lowercase hex như hai SDK tham chiếu.
- Private key legacy là base64 của XML `RSAKeyValue`; key dưới 1024 bit bị từ chối.
- Redirect bị tắt; TLS certificate verification luôn bật ngoài loopback test server.
- Token, secret, private key, code và chữ ký không xuất hiện trong `Debug`/`Display` của credential hoặc lỗi transport.
- Dependency `rsa` chỉ được dùng cho signing. Ngoại lệ audit `RUSTSEC-2023-0071` được giới hạn trong `deny.toml` vì advisory này áp dụng cho API RSA decryption không được expose hoặc gọi bởi crate.

## Kiểm tra

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-targets --all-features
```

Test tích hợp dùng HTTP/WebSocket server loopback, không dùng credential thật và không gửi lệnh giao dịch.

Offline contract fixtures trong `crates/fc-trading/tests/fixtures/` lưu request/response wire shapes đã đối chiếu. Test `signed_order_uses_verified_write_token_and_signs_transmitted_body` xác minh chữ ký trên chính byte buffer được gửi.

Sandbox smoke bị ignore mặc định và từ chối production host. Khi có sandbox credential:

```bash
SSI_FCTRADING_SANDBOX_SMOKE=I_UNDERSTAND_SANDBOX_NETWORK_CALLS \
SSI_FCTRADING_SANDBOX_API_URL=https://sandbox-api.example/ \
SSI_FCTRADING_SANDBOX_STREAM_URL=https://sandbox-stream.example/ \
SSI_FCTRADING_CONSUMER_ID='...' \
SSI_FCTRADING_CONSUMER_SECRET='...' \
SSI_FCTRADING_PRIVATE_KEY='...' \
SSI_FCTRADING_VERIFICATION_CODE='...' \
cargo test -p ssi-fc-trading --test sandbox_smoke -- --ignored --exact live_auth_read_and_stream_smoke
```

Smoke này chỉ xác thực, gọi `rateLimit`, negotiate SignalR và đóng stream. Không có live signed business POST vì FastConnect hiện không cung cấp signed dry-run/no-op endpoint; mọi signed business endpoint trong contract đều có khả năng mutation.
