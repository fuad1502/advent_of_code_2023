use std::{cmp::Ordering, fs, path::PathBuf};

struct Hand {
    cards: String,
    hand_type: HandType,
    bet: i32,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq)]
struct Label {
    what: char,
}
impl Label {
    fn new(what: char) -> Label {
        Label { what }
    }
    fn to_value(&self) -> i32 {
        match self.what {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!(),
        }
    }
}
impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_value().partial_cmp(&other.to_value())
    }
}
impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_value().cmp(&other.to_value())
    }
}

pub fn f(path: &PathBuf) -> i32 {
    let mut hands = parse_input(path);
    hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            let a_cards = a.cards.chars();
            let b_cards = b.cards.chars();
            for (a, b) in a_cards.zip(b_cards) {
                if a != b {
                    let a = Label::new(a);
                    let b = Label::new(b);
                    return a.cmp(&b);
                }
            }
            Ordering::Equal
        } else {
            (a.hand_type).cmp(&b.hand_type)
        }
    });
    let mut result = 0;
    for (rank, hand) in hands.iter().enumerate() {
        let rank = (rank + 1) as i32;
        result += rank * hand.bet;
    }
    result
}

fn parse_input(path: &PathBuf) -> Vec<Hand> {
    let mut hands = vec![];

    let input = fs::read_to_string(path).unwrap();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let cards_str = words.next().unwrap().to_string();
        let mut cards: Vec<char> = cards_str.chars().collect();
        let bet = str::parse::<i32>(words.next().unwrap()).unwrap();

        // Group cards
        cards.sort();
        let mut grouped_cards = vec![];
        grouped_cards.push((cards[0], 1));
        let mut previous_card = cards[0];
        for card in &cards[1..] {
            if *card == previous_card {
                let e = grouped_cards.last_mut().unwrap();
                e.1 += 1;
            } else {
                grouped_cards.push((*card, 1));
                previous_card = *card;
            }
        }

        // Get hand type
        grouped_cards.sort_by(|a, b| {
            if a.1 == b.1 {
                let a = Label::new(a.0);
                let b = Label::new(b.0);
                b.cmp(&a)
            } else {
                (b.1).cmp(&a.1)
            }
        });
        let hand_type = get_hand_type(&grouped_cards);

        hands.push(Hand {
            cards: cards_str,
            hand_type,
            bet,
        });
    }

    hands
}

fn get_hand_type(grouped_cards: &Vec<(char, i32)>) -> HandType {
    let first_group = grouped_cards.first().unwrap();
    if first_group.1 == 5 {
        return HandType::FiveOfAKind;
    } else {
        let second_group = grouped_cards.get(1).unwrap();
        if first_group.1 == 4 {
            return HandType::FourOfAKind;
        } else if first_group.1 == 3 && second_group.1 == 2 {
            return HandType::FullHouse;
        } else if first_group.1 == 3 {
            return HandType::ThreeOfAKind;
        } else if first_group.1 == 2 && second_group.1 == 2 {
            return HandType::TwoPair;
        } else if first_group.1 == 2 {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
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
        assert_eq!(result, 6440);
    }
}
