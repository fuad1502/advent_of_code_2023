use std::fs;
use std::path::PathBuf;

pub fn parse_input(path: PathBuf) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let input = fs::read_to_string(path).expect("Failed to read file!");
    let mut all_row_representation = vec![];
    let mut all_col_representation = vec![];
    let mut single_pattern = String::new();
    for line in input.lines() {
        // Process single pattern
        if line.len() == 0 {
            let (row_representation, col_representation) = parse_single_pattern(&single_pattern);
            all_row_representation.push(row_representation);
            all_col_representation.push(col_representation);
            single_pattern.clear();
        }
        // Insert line to single pattern
        else {
            single_pattern.push_str(line);
            single_pattern.push('\n');
        }
    }
    // Process final pattern
    let (row_representation, col_representation) = parse_single_pattern(&single_pattern);
    all_row_representation.push(row_representation);
    all_col_representation.push(col_representation);

    (all_row_representation, all_col_representation)
}

fn parse_single_pattern(input: &String) -> (Vec<i32>, Vec<i32>) {
    // Get matrix representation, number of rows and number of columns
    let mut matrix_representation = Vec::new();
    let mut number_of_rows = 0;
    let mut number_of_cols = 0;
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if number_of_rows == 0 {
                number_of_cols += 1;
            }
            row.push(c == '#');
        }
        matrix_representation.push(row);
        number_of_rows += 1;
    }
    // Initialize row and column representation
    let mut row_representation = vec![0; number_of_rows];
    let mut col_representation = vec![0; number_of_cols];
    // Get row and column representation
    for (i, row) in matrix_representation.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col {
                row_representation[i] += 2_i32.pow(j as u32);
                col_representation[j] += 2_i32.pow(i as u32);
            }
        }
    }
    (row_representation, col_representation)
}

pub fn f(all_row_representation: &Vec<Vec<i32>>, all_col_representation: &Vec<Vec<i32>>) -> usize {
    let mut total = 0;
    for (row_representation, col_representation) in all_row_representation
        .iter()
        .zip(all_col_representation.iter())
    {
        let point = calculate_point(row_representation, col_representation);
        total += point;
    }
    total
}

pub fn f2(all_row_representation: &Vec<Vec<i32>>, all_col_representation: &Vec<Vec<i32>>) -> usize {
    let mut total = 0;
    for (row_representation, col_representation) in all_row_representation
        .iter()
        .zip(all_col_representation.iter())
    {
        let point = calculate_point_2(row_representation, col_representation);
        total += point;
    }
    total
}

fn calculate_point_2(row_representation: &Vec<i32>, col_representation: &Vec<i32>) -> usize {
    // Check vertical reflection
    // Find reflection points
    let mut row_reflection_points = vec![];
    let mut previous_e = -1;
    for (i, e) in row_representation.iter().enumerate() {
        if *e == previous_e {
            row_reflection_points.push((i, false));
        } else if differ_by_one_bit(previous_e as usize, *e as usize) {
            row_reflection_points.push((i, true));
        }
        previous_e = *e;
    }
    // Check reflection points
    for (reflection_point, mut smudge_used) in row_reflection_points {
        let mut is_reflect = true;
        // Check all rows
        for i in reflection_point + 1..row_representation.len() {
            let reflection_row = 2 * (reflection_point as i32) - 1 - (i as i32);
            // Check if reflection row is out of bounds
            if reflection_row < 0 {
                break;
            }
            let reflection_row = reflection_row as usize;
            // Check if reflecting
            if row_representation[i] != row_representation[reflection_row] {
                // Check if differ by one bit if smudge is not used
                if differ_by_one_bit(
                    row_representation[i] as usize,
                    row_representation[reflection_row] as usize,
                ) && !smudge_used
                {
                    smudge_used = true;
                } else {
                    is_reflect = false;
                    break;
                }
            }
        }
        // Only return if it is reflecting and smudge is used
        if is_reflect && smudge_used {
            return reflection_point * 100;
        }
    }

    // Check horizontal reflection
    // Find reflection points
    let mut col_reflection_points = vec![];
    let mut previous_e = -1;
    for (i, e) in col_representation.iter().enumerate() {
        if *e == previous_e {
            col_reflection_points.push((i, false));
        } else if differ_by_one_bit(previous_e as usize, *e as usize) {
            col_reflection_points.push((i, true));
        }
        previous_e = *e;
    }
    // Check reflection points
    for (reflection_point, mut smudge_used) in col_reflection_points {
        let mut is_reflect = true;
        // Check all columns
        for i in reflection_point + 1..col_representation.len() {
            let reflection_col = 2 * (reflection_point as i32) - 1 - (i as i32);
            // Check if reflection col is out of bounds
            if reflection_col < 0 {
                break;
            }
            let reflection_col = reflection_col as usize;
            // Check if reflecting
            if col_representation[i] != col_representation[reflection_col] {
                // Check if differ by one bit if smudge is not used
                if differ_by_one_bit(
                    col_representation[i] as usize,
                    col_representation[reflection_col] as usize,
                ) && !smudge_used
                {
                    smudge_used = true;
                } else {
                    is_reflect = false;
                    break;
                }
            }
        }
        // Only return if it is reflecting and smudge is used
        if is_reflect && smudge_used {
            return reflection_point;
        }
    }

    panic!("Didn't found reflection point!");
}

fn differ_by_one_bit(mut a: usize, mut b: usize) -> bool {
    let mut count = 0;
    while a > 0 || b > 0 {
        if a & 1 != b & 1 {
            count += 1;
        }
        a /= 2;
        b /= 2;
    }
    count == 1
}

fn calculate_point(row_representation: &Vec<i32>, col_representation: &Vec<i32>) -> usize {
    // Check vertical reflection
    // Find reflection points
    let mut row_reflection_points = vec![];
    let mut previous_e = -1;
    for (i, e) in row_representation.iter().enumerate() {
        if *e == previous_e {
            row_reflection_points.push(i);
        }
        previous_e = *e;
    }
    // Check reflection points
    for reflection_point in row_reflection_points {
        let mut is_reflect = true;
        // Check all rows
        for i in reflection_point + 1..row_representation.len() {
            let reflection_row = 2 * (reflection_point as i32) - 1 - (i as i32);
            // Check if reflection row is out of bounds
            if reflection_row < 0 {
                break;
            }
            let reflection_row = reflection_row as usize;
            // Check if reflecting
            if row_representation[i] != row_representation[reflection_row] {
                is_reflect = false;
                break;
            }
        }
        if is_reflect {
            return reflection_point * 100;
        }
    }

    // Check horizontal reflection
    // Find reflection points
    let mut col_reflection_points = vec![];
    let mut previous_e = -1;
    for (i, e) in col_representation.iter().enumerate() {
        if *e == previous_e {
            col_reflection_points.push(i);
        }
        previous_e = *e;
    }
    // Check reflection points
    for reflection_point in col_reflection_points {
        let mut is_reflect = true;
        // Check all columns
        for i in reflection_point + 1..col_representation.len() {
            let reflection_col = 2 * (reflection_point as i32) - 1 - (i as i32);
            // Check if reflection col is out of bounds
            if reflection_col < 0 {
                break;
            }
            let reflection_col = reflection_col as usize;
            // Check if reflecting
            if col_representation[i] != col_representation[reflection_col] {
                is_reflect = false;
                break;
            }
        }
        if is_reflect {
            return reflection_point;
        }
    }

    panic!("Didn't found reflection point!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        let (all_row_representation, all_col_representation) = parse_input(path);
        let result = f(&all_row_representation, &all_col_representation);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_f2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        let (all_row_representation, all_col_representation) = parse_input(path);
        let result = f2(&all_row_representation, &all_col_representation);
        assert_eq!(result, 400);
    }
}
