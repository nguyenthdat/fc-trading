# fctrading-cli

CLI chẩn đoán an toàn cho `ssi-fc-trading`. CLI hỗ trợ kiểm tra kết nối, yêu cầu OTP, xác thực write token và đọc stream; không cung cấp thao tác giao dịch hay chuyển tài sản.

## Cài đặt

```bash
cargo install fctrading-cli
```

## Cấu hình

Tạo `.env` với các biến:

```text
SSI_FCTRADING_CONSUMER_ID
SSI_FCTRADING_CONSUMER_SECRET
SSI_FCTRADING_PRIVATE_KEY
```

## Sử dụng

```bash
fctrading-cli --help
fctrading-cli doctor
fctrading-cli request-otp
fctrading-cli verify-write
fctrading-cli stream --notify-id -1 --seconds 30
```

Dùng `fctrading-cli --env-file path/to/.env <COMMAND>` để đọc file môi trường khác.

## License

MIT
