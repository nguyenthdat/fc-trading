# SSI FastConnect Trading

[![CI](https://github.com/nguyenthdat/fc-trading/actions/workflows/ci.yml/badge.svg)](https://github.com/nguyenthdat/fc-trading/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ssi-fc-trading.svg)](https://crates.io/crates/ssi-fc-trading)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rust workspace dành cho SSI FastConnect Trading, gồm:

- `ssi-fc-trading`: thư viện async hỗ trợ xác thực, giao dịch, truy vấn tài khoản và nhận dữ liệu streaming.
- `fctrading-cli`: CLI chẩn đoán kết nối, OTP, write token và stream; không cung cấp lệnh đặt, sửa hoặc hủy lệnh giao dịch.

Đây là dự án cộng đồng, không phải SDK chính thức của SSI.

## Yêu cầu

- Rust 1.85 trở lên.
- Tài khoản và credential SSI FastConnect Trading hợp lệ.

## Cài đặt

Thêm thư viện vào ứng dụng Rust:

```bash
cargo add ssi-fc-trading secrecy
cargo add tokio --features macros,rt-multi-thread
```

Cài CLI:

```bash
cargo install fctrading-cli
```

## Cấu hình

Sao chép file mẫu và điền credential:

```bash
cp .env.example .env
```

Các biến bắt buộc:

```text
SSI_FCTRADING_CONSUMER_ID
SSI_FCTRADING_CONSUMER_SECRET
SSI_FCTRADING_PRIVATE_KEY
```

Các biến tùy chọn:

| Biến | Mặc định | Mô tả |
| --- | --- | --- |
| `SSI_FCTRADING_API_URL` | SSI production API | Ghi đè API base URL |
| `SSI_FCTRADING_STREAM_URL` | SSI production stream | Ghi đè stream base URL |
| `SSI_FCTRADING_TWO_FACTOR_TYPE` | `0` | `0`: PIN, `1`: OTP, `2`: CA |

Không commit `.env`, consumer secret hoặc private key vào repository.

## Sử dụng thư viện

```rust
use std::env;

use secrecy::SecretString;
use ssi_fc_trading::{
    AccountRequest, ClientConfig, Credentials, TradingClient, TwoFactorType,
};

#[tokio::main]
async fn main() -> ssi_fc_trading::Result<()> {
    let credentials = Credentials::from_base64_xml(
        env::var("SSI_FCTRADING_CONSUMER_ID").expect("missing consumer ID"),
        SecretString::from(
            env::var("SSI_FCTRADING_CONSUMER_SECRET").expect("missing consumer secret"),
        ),
        SecretString::from(
            env::var("SSI_FCTRADING_PRIVATE_KEY").expect("missing private key"),
        ),
    )?;

    let client = TradingClient::new(
        ClientConfig::production()?,
        credentials,
        TwoFactorType::Pin,
    )?;

    let response = client
        .stock_account_balance(&AccountRequest::new("YOUR_ACCOUNT"))
        .await?;

    println!("status={} message={}", response.status, response.message);
    Ok(())
}
```

Các thao tác thay đổi trạng thái cần xác thực write token trước:

```rust
client.verify_code("YOUR_PIN_OR_OTP").await?;
```

Client không tự retry các request thay đổi trạng thái để tránh gửi lặp giao dịch khi kết nối gián đoạn.

## Sử dụng CLI

Chạy trực tiếp từ source:

```bash
cargo run -p fctrading-cli -- --help
cargo run -p fctrading-cli -- doctor
cargo run -p fctrading-cli -- request-otp
cargo run -p fctrading-cli -- verify-write
cargo run -p fctrading-cli -- stream --notify-id -1 --seconds 30
```

Sau khi cài bằng `cargo install`, thay `cargo run -p fctrading-cli --` bằng `fctrading-cli`. Có thể chọn file môi trường khác bằng `--env-file path/to/.env`.

## Phát triển

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

## CI/CD và phát hành

- `CI` chạy format, Clippy, test, tài liệu và kiểm tra package trên pull request và mỗi lần push lên `main`.
- `Release` chạy khi push tag dạng `vMAJOR.MINOR.PATCH` và chỉ publish khi tag khớp version của cả hai crate.
- Workflow publish `ssi-fc-trading` trước, sau đó publish `fctrading-cli`.
- Tạo repository secret tên `CRATES_IO_TOKEN` chứa API token từ crates.io. Không lưu token trong source hoặc workflow.

Ví dụ phát hành version `0.1.0`:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## License

Phát hành theo giấy phép [MIT](LICENSE).
