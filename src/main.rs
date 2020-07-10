use std::env;
fn main() {
    let path = env::args().nth(1).expect("1 argument PATH required");
    // println!("Hello, world!");
    println!("path = {:?}", path);
}
