```rust

// Variable
fn main() {
    let mut x: i32 = 20;
    println!("x = {}", x);

    x = 30;
    println!("x = {}", x);

    const HANG_SO: u128 = 1_000_000;
    println!("Hang So = {}", HANG_SO);
}
```

```rust
// Shadowing
fn main(){
    let outer = 10; 
    {
        let inner = 200;
        println!("inner = {} ", inner);
        let outer = 300; 
        println!("outer = {} ", outer);

    }
    println!("outer = {} ", outer);
}
```

```rust
// tuple và print gía trị trong tuple
fn main() {
    let tup: (&str, i32) = ("hello", 100_000);
    println!("{:?}", tup);

    let (text, _integer): (&str, i32) = tup;
    let value: i32 = tup.1;

    println!("{}", value);
    println!("{}", text);
}
```


```rust
//Array
fn main() {
    let number: [i32; 3] = [100,200,300];
    let get_number = number[1];
    println!("number = {}", get_number);


    // in ra hàng với 32 số  0
    let _hashing: [i32; 32] = [0;32];
    println!("hashing = {:?}", _hashing);

    // dùng vòng lặp for 
    for i in _hashing.iter(){
        print!("{}", i );
    }
}

```


