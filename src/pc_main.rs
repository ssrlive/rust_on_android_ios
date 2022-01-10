#[path = "./lib.rs"] mod pong_lib;

use pong_lib::inner_rust_greeting;

fn main() {
    let info = "from PC";
    println!("{}", inner_rust_greeting(info));
}
