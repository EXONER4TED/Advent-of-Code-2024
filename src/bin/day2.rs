use std::fs;

fn main() {
    // setup solution output
    let mut solution1 = 0;
    let mut solution2 = 0;

    // read the input
    let puzzle_input = fs::read_to_string("./inputs/day2-input.txt")
        .expect("Should have been able to read the file");

    // split input into lines
    let reports = puzzle_input.split("\n");

    // for each report, convert the line to array of levels
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        // check base levels are safe
        if is_safe(&levels) {
            solution1 += 1;
            solution2 += 1;
        } else {
            // try removing a single element from each list of levels
            // if safe after removing one element, then add to solution2
            let mut index = 0;
            while index < levels.len() {
                let mut levels_copy = levels.to_vec();
                levels_copy.remove(index);
                if is_safe(&levels_copy) {
                    solution2 += 1;
                    break;
                }
                index += 1;
            }
        }
    }
    println!("Solution 1: {:#?}", solution1);
    println!("Solution 2: {:#?}", solution2);
}

// loop through the values in the levels
// returns if list of levels is safe
fn is_safe(levels: &Vec<i32>) -> bool {
    let mut i = 0;
    let mut is_increasing: bool = false; // default to assume increasing

    // throw out any empty lists
    if levels.len() == 0 {
        return false;
    }

    while i < levels.len() {
        // break if levels is empty
        if levels.len() == 0 {
            return false;
        }

        // if at first value of level, decide if increasing or not
        if i == 0 && levels[i + 1] > levels[i] {
            is_increasing = true;
        }

        // check for flip/flops in increase/decrease values
        // not safe if rate changes
        if (i != 0 && i != levels.len() - 1)
            && ((levels[i + 1] > levels[i] && !is_increasing)
                || (levels[i + 1] < levels[i] && is_increasing))
        {
            return false;
        }

        // check for differences within tolerance range (1-3)
        if i != levels.len() - 1
            && ((levels[i + 1] - levels[i]).abs() > 3 || (levels[i + 1] - levels[i]).abs() < 1)
        {
            return false;
        }

        i += 1;
    }
    return true;
}
