use regex::Regex;
use std::fs;

const _TRANING: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[derive(Debug, Eq, PartialEq, Clone)]
struct Match {
    start: i32,
    end: i32,
    line: i32,
    value: String,
}

fn main() {
    // regex to find numbers
    let reg_numbers = Regex::new(r"\d+").unwrap();
    let reg_symbols = Regex::new(r"(\*|\#|\+|\$|\/|\@|\%|\&|\=|\-)").unwrap();
    let content = get_file_content("src/input.txt");
    // let content = String::from(_TRANING);
    let numbers = get_matches(&content, reg_numbers);
    let symboles = get_matches(&content, reg_symbols);

    let valid_numbers = get_numbers_near_symbols(numbers, symboles);
    let result = compute_valid_numbers(valid_numbers);
    println!("Result: {}", result);
}

fn get_file_content(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}

fn get_matches(input: &str, pattern: Regex) -> Vec<Match> {
    let mut matches = Vec::new();
    for (line_index, line) in input.lines().enumerate() {
        for m in pattern.find_iter(line) {
            let range = m.range();
            matches.push(Match {
                start: range.start as i32,
                end: range.end as i32,
                value: String::from(m.as_str()),
                line: line_index as i32,
            })
        }
    }
    matches
}

fn get_numbers_near_symbols(numbers: Vec<Match>, symboles: Vec<Match>) -> Vec<Match> {
    let mut valids = Vec::new();
    'number: for number in numbers.iter() {
        for symbole in symboles.iter() {
            if (number.line - 1..number.line + 2).contains(&symbole.line)
                && (number.start - 1..number.end + 1).contains(&symbole.start)
            {
                valids.push(number.clone());
                continue 'number;
            }
        }
    }
    valids
}

fn compute_valid_numbers(valids: Vec<Match>) -> i32 {
    valids.iter().map(|m| m.value.parse::<i32>().unwrap()).sum()
}
