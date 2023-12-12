use std::{fs, path::PathBuf};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_input(path: &PathBuf) -> (Vec<Vec<char>>, (usize, usize)) {
    let input = fs::read_to_string(path).unwrap();
    let mut map = vec![];
    let mut start_position = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start_position = (i, j);
            }
            row.push(c);
        }
        map.push(row);
    }
    (map, start_position)
}

pub fn f(map: Vec<Vec<char>>, start_position: (usize, usize)) -> i32 {
    let ((mut pos_0, mut pos_1), (mut dir_0, mut dir_1)) = first_move(&map, start_position);
    let mut distance = 1;
    while pos_0 != pos_1 {
        println!("0 : {:?}, {:?}", pos_0, dir_0);
        println!("1 : {:?}, {:?}", pos_1, dir_1);
        (pos_0, dir_0) = next_move(&map, pos_0, dir_0);
        (pos_1, dir_1) = next_move(&map, pos_1, dir_1);
        distance += 1;
    }
    println!("0 : {:?}, {:?}", pos_0, dir_0);
    println!("1 : {:?}, {:?}", pos_1, dir_1);
    distance
}

fn next_move(
    map: &Vec<Vec<char>>,
    from_pos: (usize, usize),
    from_direction: Direction,
) -> ((usize, usize), Direction) {
    let c = get_char(map, from_pos);
    let to_direction = next_direction(from_direction, c);
    let mut to_pos = from_pos;
    match to_direction {
        Direction::Up => to_pos.0 -= 1,
        Direction::Down => to_pos.0 += 1,
        Direction::Right => to_pos.1 += 1,
        Direction::Left => to_pos.1 -= 1,
    }
    (to_pos, to_direction)
}

fn first_move(
    map: &Vec<Vec<char>>,
    start_position: (usize, usize),
) -> (((usize, usize), (usize, usize)), (Direction, Direction)) {
    let mut to_position = (start_position, start_position);
    let mut to_direction = (Direction::Up, Direction::Up);
    let mut found_one = false;
    // Check above
    if start_position.0 != 0 {
        let check_position = (start_position.0 - 1, start_position.1);
        let c = get_char(map, check_position);
        println!("Above: {}", c);
        if c == 'F' || c == '7' || c == '|' {
            if found_one {
                to_position.1 = check_position;
                to_direction.1 = Direction::Up;
                return (to_position, to_direction);
            } else {
                to_position.0 = check_position;
                to_direction.0 = Direction::Up;
                found_one = true;
            }
        }
    }
    // Check left
    if start_position.1 != 0 {
        let check_position = (start_position.0, start_position.1 - 1);
        let c = get_char(map, check_position);
        println!("Left: {}", c);
        if c == 'F' || c == '-' || c == 'L' {
            if found_one {
                to_position.1 = check_position;
                to_direction.1 = Direction::Left;
                return (to_position, to_direction);
            } else {
                to_position.0 = check_position;
                to_direction.0 = Direction::Left;
                found_one = true;
            }
        }
    }
    // Check below
    {
        let check_position = (start_position.0 + 1, start_position.1);
        let c = get_char(map, check_position);
        println!("Below: {}", c);
        if c == '|' || c == 'J' || c == 'L' {
            if found_one {
                to_position.1 = check_position;
                to_direction.1 = Direction::Down;
                return (to_position, to_direction);
            } else {
                to_position.0 = check_position;
                to_direction.0 = Direction::Down;
                found_one = true;
            }
        }
    }
    // Check right
    {
        let check_position = (start_position.0, start_position.1 + 1);
        let c = get_char(map, check_position);
        println!("Right: {}", c);
        if c == '7' || c == '-' || c == 'J' {
            if found_one {
                to_position.1 = check_position;
                to_direction.1 = Direction::Right;
                return (to_position, to_direction);
            } else {
                to_position.0 = check_position;
                to_direction.0 = Direction::Right;
            }
        }
    }
    panic!("Two first moves not found!");
}

fn get_char(map: &Vec<Vec<char>>, position: (usize, usize)) -> char {
    if position.0 < map.len() && position.1 < map.len() {
        *map.get(position.0).unwrap().get(position.1).unwrap()
    } else {
        'x'
    }
}

fn next_direction(from: Direction, c: char) -> Direction {
    match from {
        Direction::Up => {
            if c == '|' {
                Direction::Up
            } else if c == '7' {
                Direction::Left
            } else if c == 'F' {
                Direction::Right
            } else {
                panic!("Dead end!");
            }
        }
        Direction::Down => {
            if c == '|' {
                Direction::Down
            } else if c == 'L' {
                Direction::Right
            } else if c == 'J' {
                Direction::Left
            } else {
                panic!("Dead end!");
            }
        }
        Direction::Left => {
            if c == '-' {
                Direction::Left
            } else if c == 'F' {
                Direction::Down
            } else if c == 'L' {
                Direction::Up
            } else {
                panic!("Dead end!");
            }
        }
        Direction::Right => {
            if c == '-' {
                Direction::Right
            } else if c == 'J' {
                Direction::Up
            } else if c == '7' {
                Direction::Down
            } else {
                panic!("Dead end!");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn test_f() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        let (map, start_position) = parse_input(&path);
        assert_eq!(f(map, start_position), 4);
    }
}
