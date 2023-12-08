use std::mem::variant_count;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HandLabel {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for HandLabel {
    fn from(c: char) -> Self {
        match c {
            '2' => HandLabel::N2,
            '3' => HandLabel::N3,
            '4' => HandLabel::N4,
            '5' => HandLabel::N5,
            '6' => HandLabel::N6,
            '7' => HandLabel::N7,
            '8' => HandLabel::N8,
            '9' => HandLabel::N9,
            'T' => HandLabel::T,
            'J' => HandLabel::J,
            'Q' => HandLabel::Q,
            'K' => HandLabel::K,
            'A' => HandLabel::A,
            _ => panic!("Invalid HandLabel"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Hand {
    pub(crate) hand_type: HandType,
    pub(crate) cards: [HandLabel; 5],
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [_; 5] = s
            .chars()
            .map(HandLabel::from)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let mut c = vec![0; variant_count::<HandLabel>()];
        cards.iter().for_each(|x| c[*x as usize] += 1);
        c.sort_unstable();
        c.reverse();
        let hand_type = match (c[0], c[1]) {
            (5, 0) => HandType::FiveOfAKind,
            (4, 1) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        };
        Ok(Hand { hand_type, cards })
    }
}
