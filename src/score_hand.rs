use crate::card::{Card, Rank};

pub fn score_hand(hand: &[Card], starter: &Card) -> u8 {
    let mut cards = Vec::with_capacity(hand.len() + 1);
    cards.extend_from_slice(hand);
    cards.push(*starter);

    let quads = generate_quads(&cards);
    let trips = generate_trips(&cards);
    let pairs = generate_pairs(&cards);

    let mut score = 0;

    score += count_fifteens(&cards, &quads, &trips, &pairs);
    score += count_pairs(&pairs);
    score += count_runs(&cards);
    score += count_flush(&hand, &starter);
    score += count_nobs(&hand, &starter);

    score
}

fn generate_pairs(cards: &[Card]) -> Vec<[Card; 2]> {
    let n = cards.len();
    let mut pairs = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in i + 1..n {
            pairs.push([cards[i], cards[j]]);
        }
    }
    pairs
}

fn count_fifteens(
    cards: &[Card],
    quads: &[[Card; 4]],
    trips: &[[Card; 3]],
    pairs: &[[Card; 2]],
) -> u8 {
    let mut score: u8 = 0;

    let sum: u8 = cards.iter().map(|card| card.count_value()).sum();
    if sum == 15 {
        score += 2;
    }

    for quad in quads.iter() {
        let sum: u8 = quad.iter().map(|card| card.count_value()).sum();
        if sum == 15 {
            score += 2;
        }
    }

    for trip in trips.iter() {
        let sum: u8 = trip.iter().map(|card| card.count_value()).sum();
        if sum == 15 {
            score += 2;
        }
    }

    for pair in pairs.iter() {
        let sum: u8 = pair.iter().map(|card| card.count_value()).sum();
        if sum == 15 {
            score += 2;
        }
    }

    score
}

fn count_pairs(pairs: &[[Card; 2]]) -> u8 {
    let mut score = 0;
    for [a, b] in pairs {
        if a.rank_eq(b) {
            score += 2;
        }
    }
    score
}

fn count_runs(cards: &[Card]) -> u8 {
    let mut score = 0;

    score += count_run(cards);
    if score != 0 {
        return score;
    }

    for quad in generate_quads(cards) {
        score += count_run(&quad);
    }
    if score != 0 {
        return score;
    }

    for trip in generate_trips(cards) {
        score += count_run(&trip);
    }
    score
}

fn count_flush(hand: &[Card], starter: &Card) -> u8 {
    let suit = hand[0].suit();
    if hand[1..].iter().all(|card| card.suit() == suit) {
        if starter.suit() == suit {
            return hand.len() as u8 + 1;
        }
        return hand.len() as u8;
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

fn count_run(cards: &[Card]) -> u8 {
    let mut cards = cards.to_owned();
    cards.sort_by(|a, b| a.run_cmp(b));
    let lowest = cards[0].run_order();
    for i in 1..cards.len() {
        if cards[i].run_order() != (lowest + i as u8) {
            return 0;
        }
    }
    return cards.len() as u8;
}

fn generate_trips(cards: &[Card]) -> Vec<[Card; 3]> {
    let n = cards.len();
    let mut trips = Vec::with_capacity(n * (n - 1) * (n - 2) / 6);
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                trips.push([cards[i], cards[j], cards[k]]);
            }
        }
    }
    trips
}

fn generate_quads(cards: &[Card]) -> Vec<([Card; 4])> {
    let n = cards.len();
    let mut quads = Vec::with_capacity(n * (n - 1) * (n - 2) * (n - 3) / 24);
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    quads.push([cards[i], cards[j], cards[k], cards[l]]);
                }
            }
        }
    }
    quads
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
