use crate::card::{Card, Rank};
use itertools::Itertools;

pub fn score_hand(hand: &[Card], starter: &Card) -> u8 {
    let mut cards = hand.to_owned();
    cards.push(starter.to_owned());
    cards.sort_by(|a, b| a.run_cmp(b));

    let mut score = 0;

    score += count_fifteens(&cards);
    score += count_pairs(&cards);
    score += count_runs(&cards);
    score += count_flush(&hand, &starter);
    score += count_nobs(&hand, &starter);

    score
}

fn count_fifteens(cards: &[Card]) -> u8 {
    let card_combinations = (2..=5)
        .map(|size| cards.iter().combinations(size))
        .into_iter()
        .flatten();

    let counts =
        card_combinations.map(|cards| cards.iter().map(|card| card.count_value()).sum::<u8>());

    let score = counts.map(|count| if count == 15 { 2 } else { 0 }).sum();
    score
}

fn count_pairs(cards: &[Card]) -> u8 {
    let card_combinations = cards.iter().combinations(2);

    let score = card_combinations
        .map(|cards| if cards[0].rank_eq(cards[1]) { 2 } else { 0 })
        .sum();
    score
}

/// Assumes `cards` are already sorted
fn count_runs(cards: &[Card]) -> u8 {
    let score = cards
        .iter()
        .combinations(5)
        .map(|cards| count_run(&cards))
        .sum();
    if score != 0 {
        return score;
    }

    let score = cards
        .iter()
        .combinations(4)
        .map(|cards| count_run(&cards))
        .sum();
    if score != 0 {
        return score;
    }

    let score = cards
        .iter()
        .combinations(3)
        .map(|cards| count_run(&cards))
        .sum();
    score
}

fn count_flush(hand: &[Card], starter: &Card) -> u8 {
    let suit = hand[0].suit();
    if hand[1..].iter().all(|card| card.suit() == suit) {
        if starter.suit() == suit {
            return 5;
        }
        return 4;
    }
    0
}

fn count_nobs(hand: &[Card], starter: &Card) -> u8 {
    for card in hand.iter() {
        if card.rank() == Rank::Jack && card.suit() == starter.suit() {
            return 1;
        }
    }
    0
}

fn count_run(cards: &[&Card]) -> u8 {
    let start = cards[0].run_order();
    for i in 1..cards.len() {
        if cards[i].run_order() != (start + i as u8) {
            return 0;
        }
    }
    return cards.len() as u8;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn it_counts_fifteens() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Eight),
            Card::new(Suit::Spades, Rank::Seven),
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Hearts, Rank::Queen),
        ];
        let starter = Card::new(Suit::Hearts, Rank::King);

        let score = score_hand(&hand, &starter);
        assert_eq!(score, 2);
    }

    #[test]
    fn it_counts_pairs() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let starter = Card::new(Suit::Hearts, Rank::King);

        let score = score_hand(&hand, &starter);
        assert_eq!(score, 8);
    }

    #[test]
    fn it_counts_run_of_five() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Diamonds, Rank::Jack),
            Card::new(Suit::Spades, Rank::Ten),
        ];
        let starter = Card::new(Suit::Clubs, Rank::Nine);

        let score = score_hand(&hand, &starter);
        assert_eq!(score, 5);
    }

    #[test]
    fn it_counts_run_of_four() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Diamonds, Rank::Jack),
            Card::new(Suit::Spades, Rank::Ten),
        ];
        let starter = Card::new(Suit::Clubs, Rank::Eight);

        let score = score_hand(&hand, &starter);
        assert_eq!(score, 4);
    }

    #[test]
    fn it_counts_double_run_of_four() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let starter = Card::new(Suit::Clubs, Rank::Ten);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 10);
    }

    #[test]
    fn it_counts_run_of_three() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let starter = Card::new(Suit::Clubs, Rank::Ten);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 3);
    }

    #[test]
    fn it_counts_double_run_of_three() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Two),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let starter = Card::new(Suit::Clubs, Rank::Ten);

        let score = score_hand(&hand, &starter);
        assert_eq!(score, 8);
    }

    #[test]
    fn it_counts_triple_run_of_three() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let starter = Card::new(Suit::Clubs, Rank::Ten);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 15);
    }

    #[test]
    fn it_counts_four_card() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Spades, Rank::Ten),
            Card::new(Suit::Spades, Rank::Eight),
            Card::new(Suit::Spades, Rank::Six),
        ];
        let starter = Card::new(Suit::Clubs, Rank::Four);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 4);
    }

    #[test]
    fn it_counts_five_card() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Spades, Rank::Ten),
            Card::new(Suit::Spades, Rank::Eight),
            Card::new(Suit::Spades, Rank::Six),
        ];
        let starter = Card::new(Suit::Spades, Rank::Four);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 5);
    }

    #[test]
    fn it_counts_nobs() {
        let hand = vec![
            Card::new(Suit::Spades, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Six),
            Card::new(Suit::Clubs, Rank::Eight),
        ];
        let starter = Card::new(Suit::Spades, Rank::Four);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 1);
    }

    #[test]
    fn it_counts_the_highest_scoring_hand() {
        let hand = vec![
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Spades, Rank::Five),
            Card::new(Suit::Diamonds, Rank::Five),
            Card::new(Suit::Clubs, Rank::Five),
        ];
        let starter = Card::new(Suit::Hearts, Rank::Five);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 29);
    }

    #[test]
    fn it_counts_a_nineteen_point_hand() {
        let hand = vec![
            Card::new(Suit::Hearts, Rank::Two),
            Card::new(Suit::Clubs, Rank::Four),
            Card::new(Suit::Diamonds, Rank::Six),
            Card::new(Suit::Spades, Rank::Eight),
        ];
        let starter = Card::new(Suit::Hearts, Rank::Ten);
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 0);
    }
}
