use std::{fs, path::PathBuf};

pub fn f(path: &PathBuf) -> i32 {
    let mut result = 0;
    let games = parse_input(path);
    for game in games {
        let f = |x: &_| game.winning_numbers.iter().find(|y| x == y).is_some();
        let correct_numbers = game.owned_numbers.iter().filter(f).count() as u32;
        if correct_numbers > 0 {
            result += 2_i32.pow(correct_numbers - 1);
        }
    }
    result
}

pub fn f2(path: &PathBuf) -> i32 {
    let cards = parse_input(path);
    let mut winning_cards = vec![0; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let f = |x: &_| card.winning_numbers.iter().find(|y| x == y).is_some();
        let correct_numbers = card.owned_numbers.iter().filter(f).count();
        winning_cards[i] += 1;
        if correct_numbers > 0 {
            for j in i + 1..=i + correct_numbers {
                if j < winning_cards.len() {
                    winning_cards[j] += winning_cards[i];
                }
            }
        }
    }
    winning_cards.iter().sum()
}

struct Game {
    winning_numbers: Vec<i32>,
    owned_numbers: Vec<i32>,
}

fn parse_input(path: &PathBuf) -> Vec<Game> {
    let mut games = vec![];
    let input = fs::read_to_string(path).unwrap();
    for line in input.lines() {
        let line = line.split(":").nth(1).unwrap();
        let winning_numbers: Vec<i32> = line
            .split("|")
            .nth(0)
            .unwrap()
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string().parse().unwrap())
            .collect();
        let owned_numbers: Vec<i32> = line
            .split("|")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string().parse().unwrap())
            .collect();
        games.push(Game {
            winning_numbers,
            owned_numbers,
        });
    }
    games
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_f() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        let result = f(&path);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_f2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        let result = f2(&path);
        assert_eq!(result, 30);
    }
}
