use std::fs;

const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

fn main() {
    // open a file
    let file_name = "src/input.txt";
    let input = fs::read_to_string(file_name).unwrap();
    let input_data = input.as_str();
    // read each line
    let first = input_data
        .lines()
        .map(|line| get_first_number(line))
        .collect::<Vec<u32>>();
    let last = input_data
        .lines()
        .map(|line| line.chars().rev().collect::<String>())
        .map(|line| get_first_number(line.as_str()))
        .collect::<Vec<u32>>();
    // get numbers
    // make sum
    let sum: u32 = first
        .iter()
        .zip(last.iter())
        .map(|(a, b)| *a * 10 + *b)
        .sum();

    // print result
    println!("the result is: {}", sum)
}

fn get_first_number(string: &str) -> u32 {
    for c in string.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    0
}
