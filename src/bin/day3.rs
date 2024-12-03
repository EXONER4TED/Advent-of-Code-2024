use regex::Regex;
use std::fs;

fn main() {
    // setup solution output
    let mut solution1 = 0;

    // read the input
    let puzzle_input = fs::read_to_string("./inputs/day3-input.txt")
        .expect("Should have been able to read the file");

    // use regex to scan for mul(X,Y)
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();

    // push the captures to a list
    let mut muls = vec![];
    for (_, [m]) in re.captures_iter(&puzzle_input).map(|c| c.extract()) {
        muls.push(m);
    }

    // loop over all the mul instructions found and multiply the params
    for mul in muls {
        // use regex again to scan for just the X and Y of mul(X,Y)
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for (_, [x, y]) in re.captures_iter(&mul).map(|c| c.extract()) {
            solution1 += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }
    }
    println!("Solution 1: {:?}", solution1);

    let mut solution2 = 0;
    // use regex to scan for mul(X,Y), do(), and don't()
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    // push the captures to a list
    let mut muls_dos_donts = vec![];
    for (_, [m]) in re.captures_iter(&puzzle_input).map(|c| c.extract()) {
        muls_dos_donts.push(m);
    }

    // setup list of only the muls that matter
    let mut muls_that_count = vec![];
    let mut muls_enabled = true; // start reading with muls enabled
    for mul in muls_dos_donts {
        if mul == "do()" {
            muls_enabled = true;
        } else if mul == "don't()" {
            muls_enabled = false;
        } else if muls_enabled {
            muls_that_count.push(mul);
        }
    }

    // now do the same loop, but only for muls that count
    for mul in muls_that_count {
        // use regex again to scan for just the X and Y of mul(X,Y)
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for (_, [x, y]) in re.captures_iter(&mul).map(|c| c.extract()) {
            solution2 += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }
    }
    println!("Solution 2: {:?}", solution2);
}
