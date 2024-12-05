use std::fs;

fn main() {
    // setup solution output
    let mut solution1 = 0;

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
    let mut col_index = 0;
    while row_index < chars.len() {
        while col_index < chars[row_index].len() {
            // get the current character
            let current_char: char = chars[row_index][col_index];
            if current_char == 'X' {
                solution1 += xmas_search(&chars, row_index, col_index, current_char);
            }
            col_index += 1;
        }
        row_index += 1;
    }

    println!("Solution 1: {:?}", solution1)
}

fn xmas_search(
    chars: &Vec<Vec<char>>,
    mut row_index: usize,
    mut col_index: usize,
    mut char_to_check: char,
) -> i32 {
    // in bounds check
    if (row_index as u32) <= 0
        || (col_index as u32) <= 0
        || row_index >= chars.len()
        || col_index >= chars.len()
    {
        return 0;
    }

    // base case - we're looking for S, return 1 if we find it
    if char_to_check == 'S' {
        if chars[row_index][col_index] == char_to_check {
            println!("Found XMAS!");
            return 1;
        } else {
            return 0;
        }
    }

    // figure out next char
    match char_to_check {
        'X' => char_to_check = 'M',
        'M' => char_to_check = 'A',
        'A' => char_to_check = 'S',
        _ => char_to_check = 'S',
    }

    let mut total = 0;
    let possible_actions = ["neg", "pos", "nil"];
    for row_action in possible_actions {
        for col_action in possible_actions {
            // handle Rust getting mad if these go negative
            if (row_index == 0 && row_action == "neg") || (col_index == 0 && col_action == "neg") {
                continue;
            }
            // decide what to do for each action
            match row_action {
                "pos" => row_index += 1,
                "neg" => row_index -= 1,
                "nil" => row_index = row_index,
                _ => row_index = row_index,
            }
            match col_action {
                "pos" => col_index += 1,
                "neg" => col_index -= 1,
                "nil" => col_index = col_index,
                _ => col_index = col_index,
            }

            total += xmas_search(chars, row_index, col_index, char_to_check);
        }
    }
    println!("{:?}", total);
    return total;
}
