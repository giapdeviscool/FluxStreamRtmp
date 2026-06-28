# My RTMP Standalone Server

Dự án này là một RTMP và HTTP server độc lập được phát triển bằng ngôn ngữ Rust, sử dụng các thư viện `scuffle-rtmp` cho giao thức RTMP, `axum` cho HTTP API, và `sqlx` để kết nối với cơ sở dữ liệu PostgreSQL.

---

## Yêu cầu hệ thống (Prerequisites)

Trước khi bắt đầu, hãy đảm bảo máy của bạn đã cài đặt các công cụ sau:
1. **Rust & Cargo**: Phiên bản mới nhất (hỗ trợ Edition 2024).
2. **Docker & Docker Compose**: Để chạy cơ sở dữ liệu PostgreSQL.
3. **FFmpeg hoặc OBS Studio**: Để kiểm tra việc truyền phát (streaming) RTMP.

---

## Hướng dẫn các bước chạy Server

### Bước 1: Khởi động Cơ sở dữ liệu (Database)

Ứng dụng yêu cầu một cơ sở dữ liệu PostgreSQL để kết nối khi khởi động. Dự án đã cấu hình sẵn file [docker-compose.development.yaml](file:///home/giapphan/my-rtmp-standalone/docker-compose.development.yaml).

Chạy lệnh sau để khởi động PostgreSQL container ở chế độ nền (background):

```bash
docker compose -f docker-compose.development.yaml up -d
```

*Lưu ý:* Lệnh này sẽ tạo ra một database với thông tin sau:
- **Host**: `localhost`
- **Port**: `7654` (ánh xạ từ cổng `5432` trong container)
- **User**: `glive`
- **Password**: `glive`
- **Database Name**: `glive`

---

### Bước 2: Tạo và Cấu hình file Config

Trước khi chạy ứng dụng, bạn cần tạo file cấu hình từ file ví dụ:

```bash
cp config.example.yaml config.development.yaml
```

*(Bạn cũng có thể đặt tên khác như `config.production.yaml` tùy theo môi trường).*

Sau khi copy, bạn có thể cấu hình lại các thông số trong file `config.development.yaml` nếu cần. Cấu hình mặc định bao gồm:
```yaml
server:
  app: GLIVE
  host: 0.0.0.0
  rtmp:
    port: 1999
  http:
    port: 9876
database:
  url: postgres://glive:glive@localhost:7654/glive

logger:
  level: debug
```

---

### Bước 3: Chạy Server

Sau khi database đã hoạt động, bạn chạy ứng dụng bằng lệnh `cargo run`. 

Để truyền tham số vào chương trình Rust (thay vì truyền cho Cargo), bạn cần sử dụng ký tự gạch kép `--` trước các tham số của chương trình:

```bash
cargo run -- --config config.development.yaml
```

*Giải thích:* 
- Ký tự `--` báo hiệu cho `cargo` dừng việc đọc các tham số của riêng nó và chuyển toàn bộ các tham số phía sau (ở đây là `--config config.development.yaml`) trực tiếp cho binary của ứng dụng.
- Nếu không truyền tham số `--config`, ứng dụng sẽ tự động tìm kiếm file cấu hình mặc định là `config.development.yaml` tại thư mục gốc. Bạn cũng có thể chỉ chạy:
  ```bash
  cargo run
  ```

Khi khởi động thành công, bạn sẽ thấy log thông báo:
- Kết nối thành công đến Database.
- HTTP Server đang chạy tại `http://127.0.0.1:9876`.
- RTMP Server đang lắng nghe trên cổng `1999`.

---

## Hướng dẫn sử dụng SQLx (Cơ sở dữ liệu)

Dự án sử dụng thư viện `sqlx` để tương tác với PostgreSQL. Dưới đây là hướng dẫn thiết lập và chạy các bản phát triển cơ sở dữ liệu (migrations).

### 1. Cài đặt SQLx CLI (Khuyên dùng)

Để tạo và chạy các file migration, bạn nên cài đặt `sqlx-cli` trên máy cá nhân:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

*(Tương tự như `npm install -g`, lệnh này cài đặt công cụ CLI trên toàn hệ thống. Tùy chọn `--features postgres` giúp cài đặt nhanh hơn bằng cách chỉ cài driver cho PostgreSQL, tránh phải cài đặt thêm các thư viện phát triển của MySQL/SQLite).*

### 2. Thiết lập biến môi trường `DATABASE_URL`

`sqlx-cli` và một số macro kiểm tra compile-time của `sqlx` yêu cầu biến môi trường `DATABASE_URL`. 

Bạn có thể xuất (export) biến này trong terminal hoặc tạo file `.env` ở thư mục gốc:

```bash
export DATABASE_URL=postgres://glive:glive@localhost:7654/glive
```

Hoặc tạo file `.env`:
```env
DATABASE_URL=postgres://glive:glive@localhost:7654/glive
```

### 3. Quản lý Migrations

Nếu bạn muốn tạo bảng mới hoặc thay đổi cấu trúc cơ sở dữ liệu:

* **Tạo file migration mới (hỗ trợ reversible - cả UP và DOWN):**
  ```bash
  sqlx migrate add -r <tên_migration>
  ```
  Lệnh này sẽ tạo ra 2 file trong thư mục `migrations/` ở thư mục gốc:
  - `<timestamp>_<tên_migration>.up.sql`: Chứa câu lệnh SQL để áp dụng thay đổi (ví dụ: tạo bảng, thêm cột).
  - `<timestamp>_<tên_migration>.down.sql`: Chứa câu lệnh SQL để hoàn tác (revert) các thay đổi của file `.up.sql` tương ứng.

* **Chạy các file migration chưa được áp dụng:**
  ```bash
  sqlx migrate run
  ```

* **Hoàn tác (Revert) migration gần nhất:**
  ```bash
  sqlx migrate revert
  ```

---

## Hướng dẫn Kiểm tra & Test Server

### 1. Test truyền phát RTMP (Publish Stream)

Bạn có thể sử dụng **FFmpeg** để đẩy một luồng video giả lập (test stream) lên RTMP server:

```bash
ffmpeg -re -f lavfi -i testsrc=size=1280x720:rate=30 -f lavfi -i sine=frequency=1000 -c:v libx264 -preset veryfast -c:a aac -f flv rtmp://localhost:1999/GLIVE/test_stream
```

Hoặc cấu hình trên **OBS Studio**:
- **Service**: `Custom...`
- **Server**: `rtmp://localhost:1999/GLIVE`
- **Stream Key**: `test_stream` (hoặc bất kỳ key nào bạn chọn)

---

### 2. Test HTTP API

HTTP Server cung cấp một số endpoint để quản lý luồng stream (hiện tại đang trả về dữ liệu mẫu - mock data). Bạn có thể dùng `curl` để kiểm tra:

#### A. Tạo một Stream mới (Create Stream)
```bash
curl -X POST http://localhost:9876/stream \
  -H "Content-Type: application/json" \
  -d '{"app": "GLIVE", "stream_name": "my_awesome_stream"}'
```

#### B. Lấy danh sách các Stream (List Streams)
```bash
curl "http://localhost:9876/stream/list?page=1&limit=10"
```

#### C. Lấy thông tin chi tiết của một Stream theo ID (Get Stream)
```bash
curl http://localhost:9876/stream/01905c10-0000-7000-8000-000000000000
```
*(Thay thế UUID bằng `stream_id` thực tế nhận được từ API tạo stream).*
