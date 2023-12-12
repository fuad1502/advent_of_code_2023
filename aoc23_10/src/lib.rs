use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Copy)]
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

pub fn f(map: &Vec<Vec<char>>, start_position: (usize, usize)) -> i32 {
    let ((mut pos_0, mut pos_1), (mut dir_0, mut dir_1)) = first_move(map, start_position);
    let mut distance = 1;
    while pos_0 != pos_1 {
        (pos_0, dir_0) = next_move(map, pos_0, dir_0);
        (pos_1, dir_1) = next_move(map, pos_1, dir_1);
        distance += 1;
    }
    distance
}

pub fn f2(map: &Vec<Vec<char>>, start_position: (usize, usize)) -> i32 {
    let mut marked_map = vec![vec!['O'; map[0].len()]; map.len()];
    let ((mut pos_0, mut pos_1), (mut dir_0, mut dir_1)) = first_move(&map, start_position);
    mark_map(
        &mut marked_map,
        start_position,
        get_start_char((dir_0, dir_1)),
    );
    while pos_0 != pos_1 {
        mark_map(&mut marked_map, pos_0, get_char(map, pos_0));
        mark_map(&mut marked_map, pos_1, get_char(map, pos_1));
        (pos_0, dir_0) = next_move(&map, pos_0, dir_0);
        (pos_1, dir_1) = next_move(&map, pos_1, dir_1);
    }
    mark_map(&mut marked_map, pos_0, get_char(map, pos_0));

    let mut count = 0;
    for i in 0..map.len() {
        let mut previous_vert = 'O';
        let mut inside = false;
        for j in 0..map[0].len() {
            let c = get_char(&marked_map, (i, j));
            if c == 'O' {
                if inside {
                    mark_map(&mut marked_map, (i, j), 'I');
                    count += 1;
                }
            // vert = J, L, 7, |
            } else if c != '-' {
                if !same_vert_dir(previous_vert, c) {
                    inside = !inside;
                }
                previous_vert = c;
            }
        }
    }
    count
}

fn get_start_char(dir_pair: (Direction, Direction)) -> char {
    match dir_pair {
        (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => 'J',
        (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => 'L',
        (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => '7',
        (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => 'F',
        _ => panic!("Invalid start character"),
    }
}

fn mark_map(map: &mut Vec<Vec<char>>, position: (usize, usize), mark: char) {
    map[position.0][position.1] = mark;
}

fn same_vert_dir(a: char, b: char) -> bool {
    if (a == 'F' && b == 'J') || (a == 'L' && b == '7') {
        true
    } else {
        false
    }
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
    if position.0 < map.len() && position.1 < map[0].len() {
        map[position.0][position.1]
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
        assert_eq!(f(&map, start_position), 4);
    }

    #[test]
    fn test_f2_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test2");
        let (map, start_position) = parse_input(&path);
        assert_eq!(f2(&map, start_position), 8);
    }

    #[test]
    fn test_f2_test3() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test3");
        let (map, start_position) = parse_input(&path);
        assert_eq!(f2(&map, start_position), 10);
    }

    #[test]
    fn test_f2_test4() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test4");
        let (map, start_position) = parse_input(&path);
        assert_eq!(f2(&map, start_position), 4);
    }
}
