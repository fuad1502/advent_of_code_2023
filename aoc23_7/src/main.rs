use aoc23_7::f;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/input");
    let result = f(&path, false);
    println!("Result: {}", result);
    let result = f(&path, true);
    println!("Result (part 2): {}", result);
}
