use regex::Regex;
use std::fs;

fn main() {
    // setup solution output
    let mut solution1 = 0;
    let mut solution2 = 0;

    // read the input
    let puzzle_input = fs::read_to_string("./inputs/day3-input.txt")
        .expect("Should have been able to read the file");

    // use regex to scan for mul(X,Y)
    let re = Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)").unwrap();
    let mut results = vec![];

    for (_, [path, lineno, line]) in re.captures_iter(puzzle_input).map(|c| c.extract()) {
        results.push((path, lineno.parse::<u64>()?, line));
    }

    println!("{:?}", results)
}
