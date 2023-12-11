use std::collections::HashMap;
use std::fs;

const _TRAINING: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn main() {
    let data = get_file_content("src/input.txt");
    // let data = _TRAINING;
    let cards = parse_card(&data);
    let points = compute_points(cards);
    let total = compute_total(points);
    println!("Total: {}", total);
}

fn get_file_content(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}

fn parse_card(content: &str) -> HashMap<u32, (Vec<u32>, Vec<u32>)> {
    let mut cards = HashMap::new();
    content.lines().for_each(|line| {
        let mut split = line.split(":");
        let card_number = split
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut split = split.next().unwrap().split(" | ");
        let winner = parse_numbers(split.next().unwrap());
        let numbers = parse_numbers(split.next().unwrap());

        cards.insert(card_number, (winner, numbers));
    });
    cards
}

fn parse_numbers(text: &str) -> Vec<u32> {
    text.trim()
        .split(" ")
        .filter(|chunk| chunk.len() > 0)
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn compute_points(cards: HashMap<u32, (Vec<u32>, Vec<u32>)>) -> HashMap<u32, u32> {
    let mut points = HashMap::new();
    cards.iter().for_each(|(card, (winner, numbers))| {
        let shift = numbers.iter().filter(|n| winner.contains(n)).count();
        points.insert(card.clone(), shift as u32);
    });
    points
}

fn compute_total(points: HashMap<u32, u32>) -> u32 {
    let mut sum = 0;
    for card in points.keys() {
        sum += get_number_cards(&points, *card);
    }
    sum
}

fn get_number_cards(points: &HashMap<u32, u32>, card: u32) -> u32 {
    let mut sum = 1;
    if let Some(copies) = points.get(&card) {
        for copy in card + 1..card + 1 + copies {
            sum += get_number_cards(points, copy);
        }
    } else {
        return 0;
    }
    sum
}
