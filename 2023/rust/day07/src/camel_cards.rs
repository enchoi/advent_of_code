#[derive(PartialEq, PartialOrd, Clone, Debug, Hash, Eq, Ord)]
pub enum CamelCards {
    A,
    K,
    Q,
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
    J,
}
impl ToString for CamelCards {
    fn to_string(&self) -> String {
        match self {
            CamelCards::A => String::from("A"),
            CamelCards::K => String::from("K"),
            CamelCards::Q => String::from("Q"),
            CamelCards::T => String::from("T"),
            CamelCards::N9 => String::from("9"),
            CamelCards::E8 => String::from("8"),
            CamelCards::S7 => String::from("7"),
            CamelCards::S6 => String::from("6"),
            CamelCards::F5 => String::from("5"),
            CamelCards::F4 => String::from("4"),
            CamelCards::T3 => String::from("3"),
            CamelCards::T2 => String::from("2"),
            CamelCards::O1 => String::from("1"),
            CamelCards::J => String::from("J"),
        }
    }
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
