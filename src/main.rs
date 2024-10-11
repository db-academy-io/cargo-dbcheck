use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Cargo-dbcheck executed with arguments: {:?}", &args[1..]);
}
