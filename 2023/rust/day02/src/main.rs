use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Eq, Debug, PartialEq, Hash, Clone)]
enum Color {
    Blue,
    Green,
    Red,
}
impl Color {
    fn from(color: &str) -> Self {
        match color {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => panic!("Wrong color dude"),
        }
    }
}

const _INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn main() {
    let start = Instant::now();

    let max_dices: HashMap<Color, u32> = HashMap::from_iter([
        (Color::from("red"), 12),
        (Color::from("green"), 13),
        (Color::from("blue"), 14),
    ]);

    let file_name = "src/input.txt";
    let input = fs::read_to_string(file_name).unwrap();
    let data = input.as_str();
    // let data = _INPUT;
    let games = parse_games(data);
    let result: u32 = games
        .iter()
        .filter(|(_, v)| {
            for hashmap in *v {
                for (color, max) in hashmap {
                    if max > &max_dices[&color] {
                        return false;
                    }
                }
            }
            true
        })
        .map(|(&k, _)| k)
        .sum();
    let duration = start.elapsed();
    println!("Result is : {}", result);
    println!("Done in {:?}", duration);

    // part 2
    let mut new_games = games.clone();
    let power = new_games
        .iter_mut()
        .map(|(_, hands)| {
            let mut mini = HashMap::new();
            for hand in hands.iter_mut() {
                for (color, dices) in hand.iter_mut() {
                    mini.entry(color)
                        .and_modify(|c: &mut u32| *c = *c.max(dices))
                        .or_insert(*dices);
                }
            }
            return mini;
        })
        .collect::<Vec<HashMap<&Color, u32>>>();

    let result = power
        .iter()
        .map(|hand| hand.values().fold(1_u32, |acc, v| acc * *v))
        .sum::<u32>();
    println!("result: {}", result);
}

fn parse_games(data: &str) -> HashMap<u32, Vec<HashMap<Color, u32>>> {
    let mut hashmap = HashMap::new();
    data.lines().for_each(|line| {
        let mut split = line.split(":");
        let game_number = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let split = split.next().unwrap().split(";");
        let hands: Vec<HashMap<Color, u32>> = split.map(|hand| parse_hand(hand)).collect();
        hashmap.insert(game_number, hands);
    });

    hashmap
}

fn parse_hand(hand: &str) -> HashMap<Color, u32> {
    let mut map = HashMap::new();
    hand.split(",").for_each(|number_dice| {
        let trimed = number_dice.trim();
        let mut split = trimed.split_ascii_whitespace();
        let number = split.next().unwrap();
        let color = split.next().unwrap();
        map.insert(
            Color::from(color.trim()),
            number.trim().parse::<u32>().unwrap(),
        );
    });
    map
}
