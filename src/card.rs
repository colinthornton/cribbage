use rand::Rng;
use std::fmt;
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

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
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
