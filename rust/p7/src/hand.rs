use std::cmp::Ordering;

use enum_map::{Enum, EnumMap};

#[derive(Debug, PartialEq, Eq)]
pub struct BidHand<T: Ord> {
    pub hand: T,
    pub bid: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand(pub [Card; 5]);

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.0.cmp(&other.0),
            other => other,
        }
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut map: EnumMap<Card, u8> = EnumMap::default();
        for card in self.0.iter() {
            map[*card] += 1;
        }

        let (mut top, mut sec) = (0, 0);
        for &count in map.values() {
            if count > top {
                sec = top;
                top = count;
            } else if count > sec {
                sec = count;
            }
        }

        match (top, sec) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::Pair,
            (1, 1) => HandType::HighCard,
            _ => panic!("invalid counts {top}, {sec}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand2(pub [Card2; 5]);

impl From<Hand> for Hand2 {
    fn from(value: Hand) -> Self {
        Hand2(utils::map_fixed(value.0, |&c| Card2::from(c)))
    }
}

impl Hand2 {
    fn hand_type(&self) -> HandType {
        let mut map: EnumMap<Card2, u8> = EnumMap::default();
        for card in self.0.iter() {
            map[*card] += 1;
        }

        let mut js = 0;
        let (mut top, mut sec) = (0, 0);
        for (card, count) in map.into_iter() {
            if card == Card2::Joker {
                js = count;
                continue;
            }

            if count > top {
                sec = top;
                top = count;
            } else if count > sec {
                sec = count;
            }
        }

        match (top + js, sec) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::Pair,
            (1, 1) => HandType::HighCard,
            _ => panic!("invalid counts {top}, {sec}"),
        }
    }
}

impl PartialOrd<Hand2> for Hand2 {
    fn partial_cmp(&self, other: &Hand2) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.0.cmp(&other.0),
            other => other,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        Hand(cards)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum, PartialOrd, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum, PartialOrd, Ord)]
pub enum Card2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<Card> for Card2 {
    fn from(value: Card) -> Self {
        match value {
            Card::Two => Card2::Two,
            Card::Three => Card2::Three,
            Card::Four => Card2::Four,
            Card::Five => Card2::Five,
            Card::Six => Card2::Six,
            Card::Seven => Card2::Seven,
            Card::Eight => Card2::Eight,
            Card::Nine => Card2::Nine,
            Card::Ten => Card2::Ten,
            Card::Jack => Card2::Joker,
            Card::Queen => Card2::Queen,
            Card::King => Card2::King,
            Card::Ace => Card2::Ace,
        }
    }
}
