use std::fs;
use std::path::PathBuf;

fn f(path: &PathBuf) -> i32 {
    let input = fs::read_to_string(path).unwrap();

    let mut result = 0;
    for line in input.lines() {
        for c in line.chars() {
            if c.is_ascii_digit() {
                result += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                result += c.to_digit(10).unwrap();
                break;
            }
        }
    }
    result as i32
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/input");
    let result = f(&path);
    println!("Result: {result}");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn main_test() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        let result = f(&path);
        assert_eq!(result, 142);
    }
}
