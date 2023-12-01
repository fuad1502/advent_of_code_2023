use aoc23_1::*;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/input");
    let result = f(&path);
    println!("Result: {result}");
}
