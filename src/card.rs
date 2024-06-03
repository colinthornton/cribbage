use rand::Rng;
use std::fmt;

pub struct Deck {
    cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: [
                Card::new(Suit::Clubs, Rank::Ace),
                Card::new(Suit::Clubs, Rank::Two),
                Card::new(Suit::Clubs, Rank::Three),
                Card::new(Suit::Clubs, Rank::Four),
                Card::new(Suit::Clubs, Rank::Five),
                Card::new(Suit::Clubs, Rank::Six),
                Card::new(Suit::Clubs, Rank::Seven),
                Card::new(Suit::Clubs, Rank::Eight),
                Card::new(Suit::Clubs, Rank::Nine),
                Card::new(Suit::Clubs, Rank::Ten),
                Card::new(Suit::Clubs, Rank::Jack),
                Card::new(Suit::Clubs, Rank::Queen),
                Card::new(Suit::Clubs, Rank::King),
                Card::new(Suit::Diamonds, Rank::Ace),
                Card::new(Suit::Diamonds, Rank::Two),
                Card::new(Suit::Diamonds, Rank::Three),
                Card::new(Suit::Diamonds, Rank::Four),
                Card::new(Suit::Diamonds, Rank::Five),
                Card::new(Suit::Diamonds, Rank::Six),
                Card::new(Suit::Diamonds, Rank::Seven),
                Card::new(Suit::Diamonds, Rank::Eight),
                Card::new(Suit::Diamonds, Rank::Nine),
                Card::new(Suit::Diamonds, Rank::Ten),
                Card::new(Suit::Diamonds, Rank::Jack),
                Card::new(Suit::Diamonds, Rank::Queen),
                Card::new(Suit::Diamonds, Rank::King),
                Card::new(Suit::Hearts, Rank::Ace),
                Card::new(Suit::Hearts, Rank::Two),
                Card::new(Suit::Hearts, Rank::Three),
                Card::new(Suit::Hearts, Rank::Four),
                Card::new(Suit::Hearts, Rank::Five),
                Card::new(Suit::Hearts, Rank::Six),
                Card::new(Suit::Hearts, Rank::Seven),
                Card::new(Suit::Hearts, Rank::Eight),
                Card::new(Suit::Hearts, Rank::Nine),
                Card::new(Suit::Hearts, Rank::Ten),
                Card::new(Suit::Hearts, Rank::Jack),
                Card::new(Suit::Hearts, Rank::Queen),
                Card::new(Suit::Hearts, Rank::King),
                Card::new(Suit::Spades, Rank::Ace),
                Card::new(Suit::Spades, Rank::Two),
                Card::new(Suit::Spades, Rank::Three),
                Card::new(Suit::Spades, Rank::Four),
                Card::new(Suit::Spades, Rank::Five),
                Card::new(Suit::Spades, Rank::Six),
                Card::new(Suit::Spades, Rank::Seven),
                Card::new(Suit::Spades, Rank::Eight),
                Card::new(Suit::Spades, Rank::Nine),
                Card::new(Suit::Spades, Rank::Ten),
                Card::new(Suit::Spades, Rank::Jack),
                Card::new(Suit::Spades, Rank::Queen),
                Card::new(Suit::Spades, Rank::King),
            ],
        }
    }

    pub fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cards.len() {
            let j = rng.gen_range(i..self.cards.len());
            self.cards.swap(i, j);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Rank {
    Ace,
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
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
        }
    }
}
