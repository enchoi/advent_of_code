use super::camel_card_type::CamelCardType;
use super::camel_cards::CamelCards;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Hand {
    pub cards: Vec<CamelCards>,
    pub best_score: Vec<CamelCards>,
    pub hand_type: CamelCardType,
    pub origin: String,
}
impl Hand {
    pub fn new(cards: &str) -> Self {
        let origin = String::from(cards);
        let cards: Vec<CamelCards> = cards.chars().map(|c| CamelCards::from(c)).collect();
        let best_score = Hand::compute_best(cards.clone());
        let hand_type = CamelCardType::from(best_score.clone());
        Self {
            cards,
            best_score,
            hand_type,
            origin,
        }
    }

    pub fn compute_best(cards: Vec<CamelCards>) -> Vec<CamelCards> {
        if !cards.contains(&CamelCards::J) {
            return cards;
        }
        let mut news = Vec::new();
        let mut template = cards.clone();
        let mut joker_index = Vec::new();
        let mut available_cards = cards
            .iter()
            .filter(|&card| card != &CamelCards::J)
            .map(|card| card.clone())
            .collect::<Vec<CamelCards>>();
        if available_cards.len() == 0 {
            available_cards.push(CamelCards::A);
        }
        available_cards.sort();
        available_cards.dedup();
        for i in 0..template.len() {
            if template[i] == CamelCards::J {
                template[i] = available_cards[0].clone();
            }
        }

        for (index, card) in cards.iter().enumerate() {
            if *card == CamelCards::J {
                joker_index.push(index);
            }
        }

        for idx in 0..joker_index.len() {
            for card in available_cards.iter() {
                template[joker_index[idx]] = card.clone();
                news.push(template.clone());
                for inner in 0..idx {
                    for inner_card in available_cards.iter() {
                        template[joker_index[inner]] = inner_card.clone();
                        news.push(template.clone());
                    }
                }
            }
        }

        news[news.len() - 1].clone()
    }

    pub fn to_string(&self) -> String {
        format!(
            "Hand: {}, Best score: {}, Type: {:?}",
            self.origin,
            self.best_score
                .iter()
                .map(|card| card.to_string())
                .collect::<String>(),
            self.hand_type,
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self
                .hand_type
                .partial_cmp(&other.hand_type)
                .unwrap()
                .reverse();
        }
        for (m_card, o_card) in self.cards.iter().zip(other.cards.iter()) {
            if m_card != o_card {
                return m_card.partial_cmp(o_card).unwrap().reverse();
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}
