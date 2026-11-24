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

