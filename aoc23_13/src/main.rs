use aoc23_13::*;
use std::path::PathBuf;

fn main() {
    {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input");
        let (all_row_representation, all_col_representation) = parse_input(path);
        let result = f(&all_row_representation, &all_col_representation);
        println!("Result: {}", result);
    }
    {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/input");
        let (all_row_representation, all_col_representation) = parse_input(path);
        let result = f2(&all_row_representation, &all_col_representation);
        println!("Result (part 2): {}", result);
    }
}
