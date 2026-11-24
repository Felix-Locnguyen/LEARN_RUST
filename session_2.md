```rust
fn main() { 
    let s1 = String::from("hello"); 
    let (s2, len) = calculate_lenght(s1);
    println!("do dai cua {} la {}", s2, len);
}

fn calculate_lenght(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}
```

```rust
// cach thong dung
fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_lenght(&mut s1);
    println!("do dai cua {} la {}", s1, len);
}

fn calculate_lenght(some_string: &mut String) -> usize {
    some_string.push_str(" world");
    let length = some_string.len();
    length
}
```


```rust
// ❌ LỖI (Error E0499): Không thể mượn khả biến (`&mut s`) nhiều lần cùng một lúc trong cùng phạm vi hoạt động.
fn main() {
    let mut s = String::from("hello");
    let _r1 = &mut s; // Lần mượn khả biến thứ nhất (vẫn còn hiệu lực tới dòng 5)
    let _r2 = &mut s; // Lỗi ở đây: không thể tạo thêm tham chiếu khả biến thứ hai khi _r1 chưa kết thúc
    println!("{} {}", _r1, _r2);
}

// ✅ KHÔNG LỖI: Có thể tạo nhiều tham chiếu bất biến (`&s`) cùng một lúc vì dữ liệu chỉ được đọc, không bị thay đổi.
fn okmain() {
    let s = String::from("hello");
    let _r1 = &s; // Tham chiếu bất biến thứ nhất
    let _r2 = &s; // Tham chiếu bất biến thứ hai (hợp lệ)
    println!("{} {}", _r1, _r2);
}
```