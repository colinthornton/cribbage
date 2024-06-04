use crate::card::Card;
use itertools::Itertools;

pub fn score_the_play(cards: &[Card]) -> u8 {
    let mut score = 0;

    score += count_count(cards);
    score += count_pairs(cards);
    score += count_runs(cards);

    score
}

fn count_count(cards: &[Card]) -> u8 {
    let count = cards.iter().map(|card| card.count_value()).sum::<u8>();
    if count == 15 || count == 31 {
        return 2;
    }
    0
}

fn count_pairs(cards: &[Card]) -> u8 {
    for n in (2..=4).rev() {
        if cards.len() < n {
            continue;
        }

        if cards[cards.len() - n..]
            .iter()
            .map(|card| card.rank())
            .all_equal()
        {
            return (n * (n - 1)) as u8;
        }
    }
    0
}

fn count_runs(cards: &[Card]) -> u8 {
    let mut score = 0;
    for n in 3..=7 {
        if cards.len() < n {
            break;
        }

        let mut cards = cards[cards.len() - n..].to_owned();
        cards.sort_by(|a, b| a.run_cmp(b));
        let start = cards[0].run_order();
        if cards
            .iter()
            .map(|card| card.run_order())
            .eq(start..start + n as u8)
        {
            score = n as u8;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn it_counts_fifteens() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Eight),
            Card::new(Suit::Spades, Rank::Seven),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 2);
    }

    #[test]
    fn it_counts_thirty_ones() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Ten),
            Card::new(Suit::Spades, Rank::Jack),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Spades, Rank::Ace),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 2)
    }

    #[test]
    fn it_counts_pairs() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Hearts, Rank::Two),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 2)
    }

    #[test]
    fn it_counts_pair_royals() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Hearts, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Two),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 6);
    }

    #[test]
    fn it_counts_double_pair_royals() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Hearts, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Two),
            Card::new(Suit::Clubs, Rank::Two),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 12);
    }

    #[test]
    fn it_counts_runs_of_three() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Spades, Rank::Three),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 3)
    }

    #[test]
    fn it_counts_runs_of_four() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Spades, Rank::Three),
            Card::new(Suit::Spades, Rank::Four),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 4)
    }

    #[test]
    fn it_counts_runs_of_five() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Spades, Rank::Three),
            Card::new(Suit::Spades, Rank::Four),
            Card::new(Suit::Spades, Rank::Five),
            Card::new(Suit::Spades, Rank::Six),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 5)
    }

    #[test]
    fn it_counts_runs_of_six() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Spades, Rank::Three),
            Card::new(Suit::Spades, Rank::Four),
            Card::new(Suit::Spades, Rank::Five),
            Card::new(Suit::Spades, Rank::Six),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 6)
    }

    #[test]
    fn it_counts_runs_of_seven() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Spades, Rank::Three),
            Card::new(Suit::Spades, Rank::Four),
            Card::new(Suit::Spades, Rank::Five),
            Card::new(Suit::Spades, Rank::Six),
            Card::new(Suit::Spades, Rank::Seven),
        ];

        let score = score_the_play(&cards);
        assert_eq!(score, 7)
    }
}
