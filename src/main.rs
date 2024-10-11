use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Cargo-tester executed with arguments: {:?}", &args[1..]);
}
