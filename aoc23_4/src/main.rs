use aoc23_4::*;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/input");
    let result = f(&path);
    println!("Result: {result}");
    let result = f2(&path);
    println!("Result (part 2): {result}");
}
