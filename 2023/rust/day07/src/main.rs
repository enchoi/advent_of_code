use day07::hand::Hand;
use std::collections::HashMap;

const _TRAINING: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
KKKJ4 0";

fn main() {
    let data = _TRAINING;
    // let data = include_str!("input.txt");
    let hands: HashMap<String, i32> = parse_hands(data);
    // println!("hands: {:?}", hands);
    let hands_keys = hands.keys().map(|k| k.to_string()).collect::<Vec<_>>();
    // let hands_keys = sort_hands(hands_keys);
    let mut hands_keys = hands_keys
        .into_iter()
        .map(|hand| Hand::new(&hand))
        .collect::<Vec<Hand>>();

    hands_keys.sort();

    // println!("hands_sorted: {:?}", hands_keys);
    for hand in hands_keys.iter() {
        println!("{}", hand.to_string());
    }
    compute_score(hands, hands_keys);
}

fn parse_hands(data: &str) -> HashMap<String, i32> {
    HashMap::from_iter(
        data.lines()
            .map(|line| {
                let mut split = line.split(" ");
                let hand = split.next().unwrap().to_string();
                let score = split.next().unwrap().parse::<i32>().unwrap();
                (hand, score)
            })
            .collect::<Vec<(String, i32)>>(),
    )
}

fn compute_score(hands: HashMap<String, i32>, keys: Vec<Hand>) {
    let mut score = 0;
    for (index, key) in keys.into_iter().enumerate() {
        score += hands.get(&key.origin).unwrap() * (index + 1) as i32;
    }
    println!("Score: {}", score);
}
