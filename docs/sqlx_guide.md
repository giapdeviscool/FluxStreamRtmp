# Hướng dẫn sử dụng SQLx trong Rust: Phân biệt `query_as!` và `query_as`

Tài liệu này giúp bạn hiểu rõ cách thức hoạt động, sự khác biệt và các trường hợp áp dụng cụ thể cho từng phương thức truy vấn của thư viện **SQLx** trong Rust.

---

## 1. Bảng so sánh tổng quan

| Đặc điểm | `sqlx::query_as!` (Macro) | `sqlx::query_as::<_, T>` (Hàm) |
| :--- | :--- | :--- |
| **Cơ chế hoạt động** | **Compile-time** (Kiểm tra lỗi khi biên dịch) | **Runtime** (Kiểm tra lỗi khi chạy) |
| **Yêu cầu Database lúc build** | **Cần** (Đọc từ `.env` hoặc file offline) | **Không cần** |
| **Yêu cầu đối với Struct** | Không cần `FromRow`. Chỉ cần trùng khớp tên trường. | Bắt buộc phải `#[derive(sqlx::FromRow)]` |
| **Truyền tham số** | Truyền trực tiếp vào macro: `query_as!(T, "...", arg1, arg2)` | Dùng `.bind(arg)` |
| **Xử lý SQL động (Dynamic)** | **Không thể** (SQL phải là chuỗi hằng số) | **Có thể** (Cộng chuỗi SQL thoải mái) |

---

## 2. Chi tiết từng loại và Ví dụ

### A. `sqlx::query_as!` (Compile-time Checked Macro)
Đây là cách khuyên dùng hàng đầu trong SQLx vì nó tận dụng tối đa sức mạnh bảo mật hệ thống kiểu (type safety) của Rust.

#### Cách hoạt động:
Khi bạn chạy `cargo build` hoặc `cargo check`, SQLx sẽ dùng biến môi trường `DATABASE_URL` để kết nối vào database thật, chạy thử câu lệnh SQL của bạn để:
1. Xác thực cú pháp SQL có đúng không.
2. Kiểm tra xem các cột trả về có khớp kiểu dữ liệu với các trường trong Struct nhận dữ liệu hay không.

#### Ví dụ:
```rust
let stream = sqlx::query_as!(
    Stream,
    r#"
    SELECT id, app, status as "status: StreamStatus"
    FROM streams
    WHERE id = $1
    "#,
    id
)
.fetch_optional(&self.pool)
.await?;
```

> [!TIP]
> **Kỹ thuật Ép kiểu Enum (Type Override):**
> Vì database chỉ hiểu cột `status` là kiểu `VARCHAR`/`TEXT`, bạn cần dùng cú pháp `status as "status: StreamStatus"` để báo cho Rust tự động map chuỗi chữ thường thành Enum `StreamStatus`.

#### Ưu điểm:
* **Không bao giờ lo lỗi chính tả SQL:** Viết sai tên bảng/cột sẽ bị báo lỗi đỏ ngay lập tức khi gõ code.
* Không cần viết `#[derive(FromRow)]` cho struct.

#### Nhược điểm:
* Bắt buộc phải có database đang chạy khi build (hoặc phải cài đặt chế độ Offline bằng cách chạy lệnh `cargo sqlx prepare` trước khi deploy).
* Không thể dùng khi cần xây dựng câu query động (ví dụ: tùy chọn lọc theo điều kiện của người dùng).

---

### B. `sqlx::query_as::<_, T>` (Runtime Checked Function)
Đây là hàm Rust thông thường, hoạt động giống như cách tương tác database truyền thống trong các ngôn ngữ khác.

#### Cách hoạt động:
SQLx chỉ coi câu SQL của bạn là một chuỗi (`&str`) bình thường và không kiểm tra gì khi build. Mọi lỗi cú pháp hay sai lệch kiểu dữ liệu chỉ được phát hiện khi ứng dụng thực sự chạy đến dòng code đó.

#### Ví dụ:
```rust
// Struct bắt buộc phải có #[derive(sqlx::FromRow)]
let stream = sqlx::query_as::<_, Stream>(
    "SELECT id, app, status FROM streams WHERE id = $1"
)
.bind(id) // Phải bind thủ công
.fetch_optional(&self.pool)
.await?;
```

#### Ưu điểm:
* Cho phép ghép chuỗi SQL linh hoạt để tạo câu truy vấn động.
* Không cần kết nối database khi build ứng dụng.

#### Nhược điểm:
* Dễ xảy ra lỗi runtime nếu bạn thay đổi cấu trúc database mà quên cập nhật code Rust.

---

## 3. So sánh thêm: `query!` vs `query` (Không có `_as`)

Nếu bạn không muốn map kết quả vào một Struct có sẵn, SQLx cung cấp hai phiên bản không có chữ `_as`:

* **`sqlx::query!` (Macro):** Trả về một **Struct ẩn danh** (Anonymous Struct) được sinh tự động. Các trường của struct này chính là tên các cột bạn `SELECT`.
  ```rust
  let row = sqlx::query!("SELECT id, app FROM streams WHERE id = $1", id)
      .fetch_one(&self.pool)
      .await?;
  // row.id và row.app được tự động định nghĩa kiểu dữ liệu chuẩn xác.
  ```
* **`sqlx::query` (Hàm):** Trả về kiểu dữ liệu thô `PgRow`. Bạn phải tự bóc tách dữ liệu thủ công.
  ```rust
  let row = sqlx::query("SELECT id, app FROM streams WHERE id = $1")
      .bind(id)
      .fetch_one(&self.pool)
      .await?;
  let app: String = row.try_get("app")?; // Phải tự bóc tách
  ```

---

## 4. Nên dùng loại nào trong trường hợp nào?

### Trường hợp 1: Các câu lệnh tĩnh, cố định (Tốt nhất: `query_as!`)
* **Ví dụ:** Lấy thông tin theo ID (`find_by_id`), thêm mới (`create`), cập nhật trạng thái (`update_status`), xóa (`delete`).
* **Lý do:** Các câu SQL này không thay đổi cấu trúc. Việc dùng `query_as!` giúp bạn an tâm tuyệt đối rằng các câu lệnh cơ bản này luôn đúng cú pháp và khớp kiểu dữ liệu 100%.

### Trường hợp 2: Các câu truy vấn động có bộ lọc (Tốt nhất: `query_as`)
* **Ví dụ:** Trang tìm kiếm nâng cao có các bộ lọc không bắt buộc (chỉ lọc theo `status` nếu người dùng chọn, chỉ lọc theo `app` nếu có truyền vào...).
* **Lý do:** Bạn cần xây dựng chuỗi SQL động bằng cách cộng chuỗi:
  ```rust
  let mut sql = "SELECT * FROM streams WHERE 1=1".to_string();
  if let Some(app_name) = filter_app {
      sql.push_str(" AND app = $1");
  }
  // Macro query_as! không thể biên dịch được chuỗi động này, bắt buộc phải dùng hàm query_as.
  ```

### Trường hợp 3: Thống kê, Báo cáo hoặc kết hợp nhiều bảng (Tốt nhất: `query!`)
* **Ví dụ:** `SELECT count(*), status FROM streams GROUP BY status` hoặc `SELECT streams.*, users.name FROM streams JOIN users ...`
* **Lý do:** Các câu truy vấn này trả về các cột không khớp hoàn toàn với bất kỳ Model (Struct) nào trong hệ thống của bạn. Dùng `query!` giúp bạn lấy nhanh kết quả dạng `row.count` mà không cần tốn công định nghĩa thêm một Struct trung gian.

---

## 5. Các hàm thực thi truy vấn (Execution Methods)

Sau khi viết câu lệnh SQL bằng `query` hoặc `query!`, bạn cần gọi một trong các hàm sau ở cuối để thực thi và lấy kết quả từ database:

### A. `.execute(&pool)`
* **Kết quả trả về:** `Result<PgQueryResult>` (chứa thông tin như `.rows_affected()` để biết bao nhiêu dòng bị tác động).
* **Khi nào dùng:** Dùng cho các câu lệnh thay đổi dữ liệu mà **không cần trả về dữ liệu** (ví dụ: `INSERT` thông thường, `UPDATE` hoặc `DELETE` không có `RETURNING`).

### B. `.fetch_one(&pool)`
* **Kết quả trả về:** `Result<T>` (trả về đúng 1 dòng dữ liệu).
* **Khi nào dùng:** Khi bạn **chắc chắn** dòng đó phải tồn tại (ví dụ: `INSERT ... RETURNING *`).
* **Lưu ý:** Nếu không tìm thấy dòng nào, hàm này sẽ trả về lỗi `sqlx::Error::RowNotFound`.

### C. `.fetch_optional(&pool)`
* **Kết quả trả về:** `Result<Option<T>>` (trả về `Some(T)` nếu tìm thấy, hoặc `None` nếu không thấy).
* **Khi nào dùng:** Khi tìm kiếm theo ID hoặc theo Key mà bản ghi đó **có thể tồn tại hoặc không**. Hàm này không báo lỗi nếu không tìm thấy dữ liệu.

### D. `.fetch_all(&pool)`
* **Kết quả trả về:** `Result<Vec<T>>` (trả về danh sách các dòng dưới dạng một Vector).
* **Khi nào dùng:** Khi muốn lấy danh sách dữ liệu (ví dụ: lấy tất cả stream đang live, lấy danh sách user). Nếu không có dòng nào khớp, nó trả về Vector rỗng `Ok(vec![])` chứ không báo lỗi.

### E. `.fetch(&pool)`
* **Kết quả trả về:** `impl Stream<Item = Result<T>>` (trả về một Async Stream).
* **Khi nào dùng:** Khi bạn cần truy vấn **lượng dữ liệu cực kỳ lớn** (hàng triệu dòng). Thay vì tải toàn bộ dữ liệu vào RAM cùng một lúc như `fetch_all`, `.fetch` sẽ trả về dữ liệu theo kiểu "nhỏ giọt" (stream), giúp bạn xử lý từng dòng một để tiết kiệm bộ nhớ.

---

## 6. Các tính năng nâng cao quan trọng khác

### A. Giao dịch (Transactions)
Khi bạn cần thực hiện nhiều câu lệnh thay đổi dữ liệu đồng thời (ví dụ: trừ tiền tài khoản A và cộng tiền tài khoản B), bạn phải dùng Transaction để đảm bảo nếu một lệnh lỗi, toàn bộ các lệnh trước đó sẽ được khôi phục (rollback).

**Cách dùng trong SQLx:**
```rust
// 1. Bắt đầu Transaction
let mut tx = pool.begin().await?;

// 2. Thực thi các câu lệnh bên trong Transaction (truyền &mut *tx thay vì &pool)
sqlx::query!("INSERT INTO ...")
    .execute(&mut *tx)
    .await?;

sqlx::query!("UPDATE ...")
    .execute(&mut *tx)
    .await?;

// 3. Nếu mọi thứ ok, tiến hành Commit lưu vào DB
tx.commit().await?;

// LƯU Ý: Nếu biến `tx` bị giải phóng (drop) trước khi gọi `.commit()`, 
// SQLx sẽ tự động ROLLBACK toàn bộ thay đổi để bảo vệ dữ liệu.
```

### B. Chạy Migration tự động bằng Code
Thay vì bắt người dùng phải gõ lệnh `sqlx migrate run` thủ công bằng tay ngoài terminal, bạn có thể cấu hình cho ứng dụng Rust **tự động chạy các file migration** ngay khi vừa khởi động.

**Cách dùng trong hàm `main`:**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgres://...").await?;

    // Tự động chạy tất cả các file trong thư mục /migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("Migration thành công!");
    Ok(())
}
```

### C. Chế độ Biên dịch Offline (Offline Mode)
Như đã nói ở trên, các macro `query!` và `query_as!` yêu cầu phải có Database đang chạy lúc bạn gõ lệnh `cargo build`. Tuy nhiên, khi deploy lên server CI/CD (như GitHub Actions), bạn thường không có sẵn Database.

SQLx giải quyết việc này bằng **Offline Mode**:
1. Ở máy local (khi đang có DB chạy), bạn chạy lệnh:
   ```bash
   cargo sqlx prepare
   ```
   Lệnh này sẽ quét toàn bộ code và tạo ra một file tên là `sqlx-data.json` chứa thông tin mô tả các câu query.
2. Khi đẩy code lên Github/Server, SQLx sẽ tự động đọc file `sqlx-data.json` này để kiểm tra kiểu dữ liệu thay vì kết nối vào DB thật.
3. Để kích hoạt chế độ này, bạn chỉ cần set biến môi trường `SQLX_OFFLINE=true` lúc build trên CI/CD.

