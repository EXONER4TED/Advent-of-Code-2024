use std::fs;

fn main() {
    // setup solution output
    let mut solution1 = 0;
    let mut solution2 = 0;

    // read the input
    let puzzle_input = fs::read_to_string("./inputs/day4-input.txt")
        .expect("Should have been able to read the file");

    // split input into rows
    let rows = puzzle_input.split("\n").collect::<Vec<&str>>();

    // get the chars into a 2D array
    let mut chars = vec![vec![]];
    for row in rows {
        let row_chars: Vec<char> = row.chars().collect();
        if row_chars.len() > 0 {
            chars.push(row_chars);
        }
    }

    // loop through the rows and columns
    // start the search as each X and check each direction for 'XMAS'
    let mut row_index = 0;
    while row_index < chars.len() {
        let mut col_index = 0;
        while col_index < chars[row_index].len() {
            // get the current character
            let current_char: char = chars[row_index][col_index];
            if current_char == 'X' {
                solution1 += xmas_search(
                    &chars,
                    row_index.try_into().unwrap(),
                    col_index.try_into().unwrap(),
                );
            }
            if current_char == 'A' {
                solution2 += x_mas_search(
                    &chars,
                    row_index.try_into().unwrap(),
                    col_index.try_into().unwrap(),
                )
            }
            col_index += 1;
        }
        row_index += 1;
    }

    println!("Solution 1: {:?}", solution1);
    println!("Solution 2: {:?}", solution2);
}

fn xmas_search(chars: &Vec<Vec<char>>, row_index: i32, col_index: i32) -> i32 {
    let mut total_found = 0;

    let offsets = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for offset in offsets {
        let mut other_chars: [char; 3] = [' ', ' ', ' '];
        let mut row_offset = row_index;
        let mut col_offset = col_index;

        for attempt in 0..3 {
            row_offset += offset.0;
            col_offset += offset.1;

            if row_offset >= 0
                && row_offset < chars.len().try_into().unwrap()
                && col_offset >= 0
                && col_offset < chars[row_offset as usize].len().try_into().unwrap()
            {
                other_chars[attempt as usize] = chars[row_offset as usize][col_offset as usize];
            }
        }
        if other_chars == ['M', 'A', 'S'] {
            total_found += 1;
        }
    }

    return total_found;
}

fn x_mas_search(chars: &Vec<Vec<char>>, row_index: i32, col_index: i32) -> i32 {
    let mut total_found = 0;

    let offsets = vec![(-1, 1), (1, -1), (-1, -1), (1, 1)];
    let mut other_chars: [i32; 4] = [-2, -2, -2, -2];

    for (index, offset) in offsets.iter().enumerate() {
        let row_offset = row_index + offset.0;
        let col_offset = col_index + offset.1;

        if row_offset >= 0
            && row_offset < chars.len().try_into().unwrap()
            && col_offset >= 0
            && col_offset < chars[row_offset as usize].len().try_into().unwrap()
        {
            let current_char = chars[row_offset as usize][col_offset as usize];
            if current_char == 'M' {
                other_chars[index] = 1;
            } else if current_char == 'S' {
                other_chars[index] = -1;
            }
        }
    }
    if other_chars[0] + other_chars[1] == 0 && other_chars[2] + other_chars[3] == 0 {
        total_found += 1;
    }

    return total_found;
}
