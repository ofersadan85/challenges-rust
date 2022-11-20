use itertools::{iproduct, Itertools};
use rand::prelude::*;
use std::{collections::HashMap, fmt::Display};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, EnumIter)]
pub enum CardSuit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, EnumIter)]
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
    pub fn from_string(s: &str) -> Result<Self, CardError> {
        let parse_error = CardError::CardParseError(s.to_string());
        let mut chars = s.chars();
        let value = match chars.next() {
            Some('2') => CardValue::Two,
            Some('3') => CardValue::Three,
            Some('4') => CardValue::Four,
            Some('5') => CardValue::Five,
            Some('6') => CardValue::Six,
            Some('7') => CardValue::Seven,
            Some('8') => CardValue::Eight,
            Some('9') => CardValue::Nine,
            Some('T') => CardValue::Ten,
            Some('J') => CardValue::Jack,
            Some('Q') => CardValue::Queen,
            Some('K') => CardValue::King,
            Some('A') => CardValue::Ace,
            _ => return Err(parse_error)
        };
        let suit = match chars.next() {
            Some('♥') => CardSuit::Heart,
            Some('♦') => CardSuit::Diamond,
            Some('♣') => CardSuit::Club,
            Some('♠') => CardSuit::Spade,
            _ => return Err(parse_error)
        };
        if chars.next().is_some() { return Err(parse_error) }
        Ok(Card { value: value, suit: suit })
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
    pub cards: Vec<Card>,
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum HandKind {
    HighCard = 0,
    Pair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
    RoyalFlash = 9,
}

#[derive(Error, Debug)]
pub enum CardError {
    #[error("Only able to parse hands of 5 cards, got {0} instead")]
    CardNumbers(usize),
    #[error("Could not parse '{0}' as a card")]
    CardParseError(String),
    // #[error("Duplicate cards found in hand: '{0}'")]
    // DuplicateCards(Card)
}

pub fn eval_hand(cards: Vec<Card>) -> Result<HandKind, CardError> {
    if cards.len() != 5 {
        return Err(CardError::CardNumbers(cards.len()));
    }
    // cards.sort();  #TODO: Make cards sortable
    // cards.dedup();
    let mut suit_count: HashMap<CardSuit, u8> = HashMap::new();
    let mut value_count1: HashMap<CardValue, u8> = HashMap::new();
    for card in cards.clone() {
        suit_count
            .entry(card.suit)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        value_count1
            .entry(card.value)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    let value_count2 = value_count1.values().counts();
    let is_flush = suit_count.values().max().unwrap_or(&0) == &5;
    let (min, max) = cards
        .iter()
        .minmax_by_key(|c| c.value as u8)
        .into_option()
        .ok_or(CardError::CardNumbers(0))?;

    let is_straight =
        value_count2.get(&1).unwrap_or(&0) == &5 && max.value as u8 - min.value as u8 == 4;

    if is_straight && is_flush && max.value as u8 == 14 {
        return Ok(HandKind::RoyalFlash);
    } else if is_straight && is_flush {
        return Ok(HandKind::StraightFlush);
    } else if is_flush {
        return Ok(HandKind::Flush);
    } else if is_straight {
        return Ok(HandKind::Straight);
    }

    let result = (
        value_count2.get(&2).unwrap_or(&0).to_owned(),
        value_count2.get(&3).unwrap_or(&0).to_owned(),
        value_count2.get(&4).unwrap_or(&0).to_owned(),
    );
    Ok(match result {
        (0, 0, 1) => HandKind::FourOfAKind,
        (0, 1, 0) => HandKind::ThreeOfAKind,
        (1, 1, 0) => HandKind::FullHouse,
        (2, 0, 0) => HandKind::TwoPairs,
        (1, 0, 0) => HandKind::Pair,
        _ => HandKind::HighCard,
    })
}
