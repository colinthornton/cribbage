use rand::Rng;
use std::{cmp, fmt};
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
                cards.push(Card::new(suit, rank));
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
            return Err(InsufficientCardsError);
        }

        let mut cards = Vec::with_capacity(n.into());
        for _ in 0..n {
            cards.push(self.draw().unwrap());
        }
        Ok(cards)
    }

    /// Draw a single card from the deck
    pub fn draw(&mut self) -> Result<Card, InsufficientCardsError> {
        self.cards.pop().ok_or(InsufficientCardsError)
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cards.len() {
            let j = rng.gen_range(i..self.cards.len());
            self.cards.swap(i, j);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// The value used when counting during the play or fifteens during the show
    pub fn count_value(&self) -> u8 {
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

    /// The order of a card when sorted for a run
    pub fn run_order(&self) -> u8 {
        match self.rank {
            Rank::Ace => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
            Rank::Nine => 8,
            Rank::Ten => 9,
            Rank::Jack => 10,
            Rank::Queen => 11,
            Rank::King => 12,
        }
    }

    pub fn run_cmp(&self, other: &Self) -> cmp::Ordering {
        if self.run_order() > other.run_order() {
            return cmp::Ordering::Greater;
        } else if self.run_order() < other.run_order() {
            return cmp::Ordering::Less;
        }
        cmp::Ordering::Equal
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Clone, Copy, EnumCount, EnumIter, Display, PartialEq, Debug)]
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

#[derive(Clone, Copy, EnumCount, EnumIter, Display, PartialEq, Debug)]
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
pub struct InsufficientCardsError;
