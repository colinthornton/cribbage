use rand::Rng;
use std::{cmp::Ordering, fmt};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(Suit::COUNT * Rank::COUNT);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card { suit, rank });
            }
        }
        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }

    /// Draw `n` cards from the deck
    pub fn draw_n(&mut self, n: u8) -> Result<Vec<Card>, InsufficientCardsError> {
        let remaining = self.cards.len() as u8;
        if remaining < n.into() {
            return Err(InsufficientCardsError { remaining });
        }

        let mut cards = Vec::with_capacity(n.into());
        for _ in 0..n {
            cards.push(self.draw().unwrap());
        }
        Ok(cards)
    }

    /// Draw a single card from the deck
    pub fn draw(&mut self) -> Result<Card, InsufficientCardsError> {
        self.cards
            .pop()
            .ok_or(InsufficientCardsError { remaining: 0 })
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cards.len() {
            let j = rng.gen_range(i..self.cards.len());
            self.cards.swap(i, j);
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self.rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
        }
    }

    pub fn order(&self) -> u8 {
        match self.rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value() > other.value() {
            return Some(Ordering::Greater);
        } else if self.value() < other.value() {
            return Some(Ordering::Less);
        }
        Some(Ordering::Equal)
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, EnumCount, EnumIter, Display)]
pub enum Suit {
    #[strum(to_string = "♣")]
    Clubs,
    #[strum(to_string = "♦")]
    Diamonds,
    #[strum(to_string = "♥")]
    Hearts,
    #[strum(to_string = "♠")]
    Spades,
}

#[derive(Clone, Copy, EnumCount, EnumIter, Display)]
pub enum Rank {
    #[strum(to_string = "A")]
    Ace,
    #[strum(to_string = "2")]
    Two,
    #[strum(to_string = "3")]
    Three,
    #[strum(to_string = "4")]
    Four,
    #[strum(to_string = "5")]
    Five,
    #[strum(to_string = "6")]
    Six,
    #[strum(to_string = "7")]
    Seven,
    #[strum(to_string = "8")]
    Eight,
    #[strum(to_string = "9")]
    Nine,
    #[strum(to_string = "10")]
    Ten,
    #[strum(to_string = "J")]
    Jack,
    #[strum(to_string = "Q")]
    Queen,
    #[strum(to_string = "K")]
    King,
}

#[derive(Debug)]
pub struct InsufficientCardsError {
    remaining: u8,
}
