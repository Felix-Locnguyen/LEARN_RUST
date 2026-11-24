
// tuple và print gía trị trong tuple
fn main() {
    let tup: (&str, i32) = ("hello", 100_000);
    println!("{:?}", tup);

    let (text, _integer): (&str, i32) = tup;
    let value: i32 = tup.1;

    println!("{}", value);
    println!("{}", text);
}


// Variable
fn Variable() {
    let mut x: i32 = 20;
    println!("x = {}", x);

    x = 30;
    println!("x = {}", x);

    const HANG_SO: u128 = 1_000_000;
    println!("Hang So = {}", HANG_SO);
}


// Shadowing
fn Shadowing(){
    let outer = 10; 
    {
        let inner = 200;
        println!("inner = {} ", inner);
        let outer = 300; 
        println!("outer = {} ", outer);

    }
    println!("outer = {} ", outer);
}
