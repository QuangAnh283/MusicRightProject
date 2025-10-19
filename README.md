# 🎵 Soroban Royalty Contract Project

## 📘 Giới thiệu

Đây là dự án **Smart Contract** được phát triển bằng **Rust + Soroban SDK (v23.0.2)** trên mạng **Stellar Testnet**.
Hợp đồng có chức năng **quản lý phân chia tiền bản quyền (royalty)** cho các bên tham gia một bản nhạc (hoặc tài sản kỹ thuật số).

Dự án được xây dựng theo **cấu trúc chuẩn Soroban**, dễ dàng mở rộng thêm nhiều hợp đồng hoặc tích hợp frontend React.

---

## 🏗️ Cấu trúc dự án

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs        # Mã nguồn hợp đồng chính (RoyaltyContract)
│       │   └── test.rs       # Bộ kiểm thử hợp đồng
│       └── Cargo.toml        # Cấu hình crate cho hợp đồng
├── Cargo.toml                # Workspace chính cho toàn dự án
└── README.md                 # Tài liệu hướng dẫn (file này)
```

---

## ⚙️ Công nghệ sử dụng

* **Rust** (phiên bản 1.90.0)
* **Soroban SDK** v23.0.2
* **Stellar CLI / Soroban CLI**
* **WASM** cho việc biên dịch hợp đồng
* **Stellar Testnet** để triển khai và thử nghiệm

---

## 🚀 Cài đặt môi trường

### 1️⃣ Cài Rust và Soroban CLI

```bash
rustup update
cargo install --locked soroban-cli
```

Kiểm tra phiên bản:

```bash
rustc --version
soroban --version
```

### 2️⃣ Thiết lập tài khoản trên Testnet

```bash
soroban keys generate test
soroban keys fund test --network testnet
```

### 3️⃣ Kiểm tra cấu hình tài khoản

```bash
soroban keys list
```

---

## 🧱 Build hợp đồng

Từ thư mục gốc của dự án, chạy:

```bash
soroban contract build
```

File `.wasm` sẽ được tạo tại:

```
contracts/hello_world/target/wasm32v1-none/release/hello_world.wasm
```

---

## 🧪 Kiểm thử hợp đồng

Từ thư mục `contracts/hello_world`, chạy:

```bash
cargo test
```

Nếu mọi thứ hợp lệ, kết quả sẽ hiển thị:

```
running n tests
test result: ok. 2 passed; 0 failed;...
```

---

## ☁️ Triển khai hợp đồng lên Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32v1-none/release/hello_world.wasm \
  --source-account test \
  --network testnet \
  --alias royalty_contract
```

Sau khi thành công, CLI sẽ trả về **Contract ID** (bắt đầu bằng chữ `C...`), ví dụ:

```
Contract ID: CDZUXZP4LQKYZB4JG2B...
```

---

## 💡 Các hàm chính trong hợp đồng

| Hàm                 | Mô tả                                   | Ghi chú                           |
| ------------------- | --------------------------------------- | --------------------------------- |
| `register_identity` | Liên kết hash danh tính với địa chỉ ví  | Cần xác thực bởi chính địa chỉ đó |
| `resolve_identity`  | Truy ngược hash danh tính về địa chỉ    | Trả về lỗi nếu không tồn tại      |
| `register_track`    | Đăng ký metadata + các bên hưởng quyền  | Tổng cổ phần = 10000              |
| `get_track_meta`    | Lấy thông tin metadata của bài hát      |                                   |
| `pay_royalty`       | Tự động chia tiền bản quyền cho các bên | Chia theo tỷ lệ cổ phần           |

---

## 🖥️ (Tùy chọn) Frontend tích hợp

Frontend React (hiện chưa bao gồm trong repo) có thể thêm trực tiếp ở cấp root của dự án:

```
/frontend
├── src/
├── package.json
└── ...
```

Frontend có thể dùng SDK như:

* `stellar-sdk`
* `soroban-client`
  để gọi các hàm của hợp đồng thông qua contract ID hoặc alias (`royalty_contract`).

---

## 📜 Giấy phép

Dự án được phát hành dưới giấy phép **MIT License**.
Bạn có thể tự do sao chép, chỉnh sửa và triển khai với ghi nhận nguồn gốc.

---

## 👨‍💻 Tác giả

* **Tên:** Bạch Quang Anh, Phạm Thế Minh, Hoàng Hà Phong.
* **Trường:** Công nghệ thông tin, Đại Học Phenikaa.
* **Ngành:** Công nghệ Thông tin Việt Nhật.


