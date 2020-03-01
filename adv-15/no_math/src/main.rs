use std::env;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Couldn't read input file :(");
    println!("Hello, world! {}", contents);
}
