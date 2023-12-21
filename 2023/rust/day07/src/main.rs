use std::collections::HashMap;

use counter::Counter;

const _TRAINING: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(PartialEq, PartialOrd, Debug)]
enum CamelCardType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl From<&str> for CamelCardType {
    fn from(value: &str) -> Self {
        let counter = value.chars().collect::<Counter<_>>();
        match counter.values().collect::<Vec<_>>() {
            value if value.contains(&&5) => CamelCardType::FiveOfAKind,
            value if value.contains(&&4) => CamelCardType::FourOfAKind,
            value if value.contains(&&3) && value.contains(&&2) => CamelCardType::FullHouse,
            value if value.contains(&&3) => CamelCardType::ThreeOfAKind,
            value if value.iter().collect::<Counter<_>>().get(&&2) == Some(&2) => {
                CamelCardType::TwoPair
            }
            value if value.contains(&&2) => CamelCardType::OnePair,
            _ => CamelCardType::HighCard,
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CamelCards {
    A,
    K,
    Q,
    J,
    T,
    N9,
    E8,
    S7,
    S6,
    F5,
    F4,
    T3,
    T2,
    O1,
}
impl From<char> for CamelCards {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::N9,
            '8' => Self::E8,
            '7' => Self::S7,
            '6' => Self::S6,
            '5' => Self::F5,
            '4' => Self::F4,
            '3' => Self::T3,
            '2' => Self::T2,
            '1' => Self::O1,
            _ => panic!("Should not have this kind of card"),
        }
    }
}

fn main() {
    // let data = _TRAINING;
    let data = include_str!("input.txt");
    let hands: HashMap<String, i32> = parse_hands(data);
    // println!("hands: {:?}", hands);
    let hands_keys = hands.keys().map(|k| k.to_string()).collect::<Vec<_>>();
    let hands_keys = sort_hands(hands_keys);
    println!("hands_sorted: {:#?}", hands_keys);
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

fn sort_hands(hands: Vec<String>) -> Vec<String> {
    let mut hands = Vec::from(hands);
    hands.sort_by(|a, b| {
        let (first, second) = (
            CamelCardType::from(a.as_str()),
            CamelCardType::from(b.as_str()),
        );
        if first == second {
            for (char_a, char_b) in a.chars().zip(b.chars()) {
                let (card_a, card_b) = (CamelCards::from(char_a), CamelCards::from(char_b));
                if card_a == card_b {
                    continue;
                }
                return card_a.partial_cmp(&card_b).unwrap().reverse();
            }
        }
        return first.partial_cmp(&second).unwrap().reverse();
    });
    hands
}

fn compute_score(hands: HashMap<String, i32>, keys: Vec<String>) {
    let mut score = 0;
    for (index, key) in keys.into_iter().enumerate() {
        score += hands.get(&key).unwrap() * (index + 1) as i32;
    }
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {

    use super::sort_hands;
    use super::CamelCardType;

    #[test]
    fn CamelCardType_test() {
        assert_eq!(CamelCardType::from("AAAAA"), CamelCardType::FiveOfAKind);
        assert_eq!(CamelCardType::from("AAAA1"), CamelCardType::FourOfAKind);
        assert_eq!(CamelCardType::from("AAA22"), CamelCardType::FullHouse);
        assert_eq!(CamelCardType::from("AAA12"), CamelCardType::ThreeOfAKind);
        assert_eq!(CamelCardType::from("11A22"), CamelCardType::TwoPair);
        assert_eq!(CamelCardType::from("11234"), CamelCardType::OnePair);
        assert_eq!(CamelCardType::from("12345"), CamelCardType::HighCard);
    }

    #[test]
    fn CamelCardType_ordering() {
        assert_eq!(
            sort_hands(vec![String::from("KTJJT"), String::from("QQQJA")]),
            &["KTJJT", "QQQJA"]
        )
    }

    #[test]
    fn CamelCardType_ordering2() {
        assert_eq!(
            sort_hands(vec![String::from("AJQQQ"), String::from("QQQJA")]),
            &["QQQJA", "AJQQQ"]
        )
    }

    #[test]
    fn CamelCardType_ordering3() {
        assert_eq!(
            sort_hands(vec![String::from("22226"), String::from("QQQ8Q")]),
            &["22226", "QQQ8Q"]
        )
    }
}
