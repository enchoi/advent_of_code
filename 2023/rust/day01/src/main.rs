use std::collections::HashMap;
use std::fs;

const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    // open a file
    let file_name = "src/input.txt";
    let input = fs::read_to_string(file_name).unwrap();
    let input_data = input.as_str();
    // let input_data = INPUT;
    let digits = input_data
        // read each line
        .lines()
        // get numbers
        .map(|line| get_numbers(line))
        .collect::<Vec<(u32, u32)>>();
    // make sum
    let sum: u32 = digits.iter().map(|(a, b)| *a * 10 + *b).sum();

    // print result
    println!("the result is: {}", sum)
}

fn get_numbers(string: &str) -> (u32, u32) {
    let mut map = HashMap::new();
    for (idx, to_find) in DIGITS.iter().enumerate() {
        string.match_indices(to_find).for_each(|matches| {
            map.insert(matches.0, idx + 1);
        })
    }

    for (idx, c) in string.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            map.insert(idx, digit as usize);
        }
    }
    let first = map.keys().min().unwrap();
    let last = map.keys().max().unwrap();
    return (map[first] as u32, map[last] as u32);
}
