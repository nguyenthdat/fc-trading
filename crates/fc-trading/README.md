# ssi-fc-trading

Async Rust client cho SSI FastConnect Trading.

Các invariant chính:

- 33 catalogued operations theo package Python 2.5.3, cộng AccessToken path cho authentication.
- Read token và write token tách biệt.
- Signed POST dùng chính JSON bytes được truyền trên wire.
- Required core response fields decode strict; API mở rộng có `ApiResponse<serde_json::Value>` khi SDK tham chiếu không định nghĩa schema response.
- Classic SignalR 1.3 qua `BroadcastHubV2`, reconnect có cancellation và backpressure.
- TLS an toàn, không retry POST, không log secret.

Xem `../../README.md` để biết cấu hình, ví dụ và lệnh kiểm tra.
