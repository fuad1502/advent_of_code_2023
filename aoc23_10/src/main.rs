use aoc23_10::*;
use std::path::PathBuf;

fn main() {
    {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input");
        let (map, start_position) = parse_input(&path);
        println!("Result: {}", f(&map, start_position));
    }
    {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input");
        let (map, start_position) = parse_input(&path);
        println!("Result: {}", f2(&map, start_position));
    }
}
