use itertools::{iproduct, Itertools};
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy, EnumIter)]
pub enum CardSuit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, EnumIter)]
pub enum CardValue {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

impl Card {
    pub fn new(value: CardValue, suit: CardSuit) -> Self {
        Card {
            value: value,
            suit: suit,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self.suit {
            CardSuit::Heart => "♥".to_string(),
            CardSuit::Diamond => "♦".to_string(),
            CardSuit::Club => "♣".to_string(),
            CardSuit::Spade => "♠".to_string(),
        };
        let value = match self.value {
            CardValue::Ten => "T".to_string(),
            CardValue::Jack => "J".to_string(),
            CardValue::Queen => "Q".to_string(),
            CardValue::King => "K".to_string(),
            CardValue::Ace => "A".to_string(),
            _ => (self.value as u8).to_string(),
        };
        write!(f, "{}{}", value, suit)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: iproduct!(CardValue::iter(), CardSuit::iter())
                .map(|(value, suit)| Card::new(value, suit))
                .collect(),
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cards.iter().join(" "))
    }
}
