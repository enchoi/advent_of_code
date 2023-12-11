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
    symbol: Option<Box<Match>>,
}

fn main() {
    // regex to find numbers
    let reg_numbers = Regex::new(r"\d+").unwrap();
    let reg_symbols = Regex::new(r"(\*|\#|\+|\$|\/|\@|\%|\&|\=|\-)").unwrap();
    let content = get_file_content("src/input.txt");
    // let content = String::from(_TRANING);
    let numbers = get_matches(&content, reg_numbers);
    let symbols = get_matches(&content, reg_symbols);

    let valid_numbers = get_numbers_near_symbols(numbers, symbols);
    let result = compute_valid_numbers(valid_numbers.clone());
    println!("Result: {}", result);
    // part 2
    let gears = find_gears(valid_numbers);
    let sum_gears = compute_sum_gears(gears);
    println!("Sum gears = {}", sum_gears);
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
                symbol: None,
            })
        }
    }
    matches
}

fn get_numbers_near_symbols(numbers: Vec<Match>, symbols: Vec<Match>) -> Vec<Match> {
    let mut valids = Vec::new();
    'number: for number in numbers.iter() {
        for symbol in symbols.iter() {
            if (number.line - 1..number.line + 2).contains(&symbol.line)
                && (number.start - 1..number.end + 1).contains(&symbol.start)
            {
                valids.push(Match {
                    symbol: Some(Box::new(symbol.clone())),
                    ..number.clone()
                });
                continue 'number;
            }
        }
    }
    valids
}

fn compute_valid_numbers(valids: Vec<Match>) -> i32 {
    valids.iter().map(|m| m.value.parse::<i32>().unwrap()).sum()
}

fn find_gears(number: Vec<Match>) -> Vec<(Match, Match)> {
    let mut gears = Vec::new();
    let mut found = Vec::new();
    for (index, m1) in number.iter().enumerate() {
        if m1.symbol.clone().unwrap().value != String::from("*") {
            continue;
        }
        for m2 in number.iter().skip(index + 1) {
            if m1.symbol == m2.symbol {
                if !(found.contains(&m1)) {
                    gears.push((m1.clone(), m2.clone()));
                    found.push(&m1);
                }
            }
        }
    }
    gears
}

fn compute_sum_gears(gears: Vec<(Match, Match)>) -> u32 {
    gears
        .iter()
        .map(|gear| parse_match(&gear.0) * parse_match(&gear.1))
        .sum::<u32>()
}

fn parse_match(m: &Match) -> u32 {
    m.value.parse::<u32>().unwrap()
}
