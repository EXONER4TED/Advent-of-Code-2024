use std::fs;

fn main() {
    // setup solution1 output
    let mut solution1 = 0;

    // read the input
    let puzzle_input = fs::read_to_string("./inputs/day1-input.txt")
        .expect("Should have been able to read the file");

    // split input into lines
    let input_lines = puzzle_input.split("\n");

    // split puzzle input into the 2 lists
    let mut l_list: Vec<i32> = Vec::new();
    let mut r_list: Vec<i32> = Vec::new();
    for vals in input_lines {
        let l_side = vals.split_whitespace().next().unwrap_or("");
        let r_side = vals.split_whitespace().last().unwrap_or("");
        // ensure no empty strings
        if !l_side.is_empty() && !r_side.is_empty() {
            l_list.push(l_side.parse().unwrap());
            r_list.push(r_side.parse().unwrap());
        }
    }

    // sort the lists
    l_list.sort();
    r_list.sort();

    // get distances
    let mut i = 0;
    loop {
        solution1 += (r_list[i] - l_list[i]).abs(); // absolute value the two
        i += 1;
        if i >= r_list.len() {
            break;
        }
    }
    println!("Solution 1: {:#?}", solution1);

    // setup solution2 output
    let mut solution2 = 0;

    // loop over left list to count appearance in right list
    for l_val in &l_list {
        let mut count = 0;
        for r_val in &r_list {
            if l_val == r_val {
                count += 1;
            }
        }
        solution2 += l_val * count;
    }
    println!("Solution 2: {:#?}", solution2);
}
