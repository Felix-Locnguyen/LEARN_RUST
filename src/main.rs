fn main() { 
    let s1 = String::from("hello"); 
    let (s2, len) = calculate_lenght(s1);
    println!("do dai cua {} la {}", s2, len);
}

fn calculate_lenght(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}