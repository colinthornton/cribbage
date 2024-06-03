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
        if a.rank == b.rank {
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
    let suit = hand[0].suit;
    if hand[1..].iter().all(|card| card.suit == suit) {
        if starter.suit == suit {
            return hand.len() as u8 + 1;
        }
        return hand.len() as u8;
    }
    0
}

fn count_nobs(hand: &[Card], starter: &Card) -> u8 {
    for card in hand.iter() {
        if card.rank == Rank::Jack && card.suit == starter.suit {
            return 1;
        }
    }
    0
}

fn count_run(cards: &[Card]) -> u8 {
    let mut cards = cards.to_owned();
    cards.sort();
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
            Card {
                suit: Suit::Spades,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
        ];
        let starter = Card {
            suit: Suit::Hearts,
            rank: Rank::King,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 2);
    }

    #[test]
    fn it_counts_pairs() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::King,
            },
        ];
        let starter = Card {
            suit: Suit::Hearts,
            rank: Rank::King,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 8);
    }

    #[test]
    fn it_counts_run_of_five() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
        ];
        let starter = Card {
            suit: Suit::Clubs,
            rank: Rank::Nine,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 5);
    }

    #[test]
    fn it_counts_run_of_four() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
        ];
        let starter = Card {
            suit: Suit::Clubs,
            rank: Rank::Eight,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 4);
    }

    #[test]
    fn it_counts_double_run_of_four() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
        ];
        let starter = Card {
            suit: Suit::Clubs,
            rank: Rank::Ten,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 10);
    }

    #[test]
    fn it_counts_run_of_three() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
        ];
        let starter = Card {
            suit: Suit::Clubs,
            rank: Rank::Ten,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 3);
    }

    #[test]
    fn it_counts_double_run_of_three() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
        ];
        let starter = Card {
            suit: Suit::Clubs,
            rank: Rank::Ten,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 8);
    }

    #[test]
    fn it_counts_triple_run_of_three() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
        ];
        let starter = Card {
            suit: Suit::Clubs,
            rank: Rank::Ten,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 15);
    }

    #[test]
    fn it_counts_four_card_flush() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Six,
            },
        ];
        let starter = Card {
            suit: Suit::Clubs,
            rank: Rank::Four,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 4);
    }

    #[test]
    fn it_counts_five_card_flush() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Six,
            },
        ];
        let starter = Card {
            suit: Suit::Spades,
            rank: Rank::Four,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 5);
    }

    #[test]
    fn it_counts_nobs() {
        let hand = vec![
            Card {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Eight,
            },
        ];
        let starter = Card {
            suit: Suit::Spades,
            rank: Rank::Four,
        };
        let score = score_hand(&hand, &starter);
        assert_eq!(score, 1);
    }
}
