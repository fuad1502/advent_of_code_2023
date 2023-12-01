use std::fs;
use std::path::PathBuf;

pub fn f(path: &PathBuf) -> i32 {
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

pub fn f2(path: &PathBuf) -> i32 {
    let input = fs::read_to_string(path).unwrap();

    let mut dm = vec![];
    let mut dm_rev = vec![];
    for i in 0..9 {
        let i = i + 1 as i32;
        dm.push(DigitMatcher::new(i, false));
        dm_rev.push(DigitMatcher::new(i, true));
    }

    let mut result = 0;
    for line in input.lines() {
        for c in line.chars() {
            if c.is_ascii_digit() {
                result += c.to_digit(10).unwrap() * 10;
                break;
            }
            let mut found = false;
            for i in 0..9 {
                if dm[i].match_character(c) {
                    result += ((i + 1) * 10) as u32;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                result += c.to_digit(10).unwrap();
                break;
            }
            let mut found = false;
            for i in 0..9 {
                if dm_rev[i].match_character(c) {
                    result += (i + 1) as u32;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        for i in 0..9 {
            dm[i].reset();
            dm_rev[i].reset();
        }
    }
    result as i32
}

struct DigitMatcher {
    digit_string: String,
    match_idx: usize,
}

impl DigitMatcher {
    fn new(digit: i32, reverse: bool) -> DigitMatcher {
        let mut digit_string = match digit {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => panic!(),
        }
        .to_string();
        if reverse {
            digit_string = digit_string.chars().rev().collect();
        }
        DigitMatcher {
            digit_string,
            match_idx: 0,
        }
    }

    fn match_character(&mut self, c: char) -> bool {
        if self.match_idx < self.digit_string.len() {
            if self.digit_string.chars().nth(self.match_idx).unwrap() == c {
                self.match_idx += 1;
            } else if self.digit_string.chars().nth(0).unwrap() == c {
                self.match_idx = 1;
            } else {
                self.match_idx = 0;
            }
        }
        if self.match_idx == self.digit_string.len() {
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        self.match_idx = 0;
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_f() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        let result = f(&path);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_f2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test_2");
        let result = f2(&path);
        assert_eq!(result, 281);
    }
}
