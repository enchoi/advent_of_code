use super::camel_cards::CamelCards;
use counter::Counter;

#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub enum CamelCardType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl CamelCardType {
    pub fn from(hand: Vec<CamelCards>) -> Self {
        let counter = hand
            .clone()
            .into_iter()
            .collect::<Counter<CamelCards>>()
            .values()
            .map(|v| *v)
            .collect::<Counter<usize>>();
        if counter.contains_key(&5) {
            return CamelCardType::FiveOfAKind;
        }
        if counter.contains_key(&4) {
            return CamelCardType::FourOfAKind;
        }
        if counter.contains_key(&3) & counter.contains_key(&2) {
            return CamelCardType::FullHouse;
        }
        if counter.contains_key(&3) {
            return CamelCardType::ThreeOfAKind;
        }
        if counter.get(&2) == Some(&2) {
            return CamelCardType::TwoPair;
        }
        if counter.contains_key(&2) {
            return CamelCardType::OnePair;
        }
        CamelCardType::HighCard
    }
}
