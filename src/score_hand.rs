use crate::card::Card;

pub fn score_hand(hand: &[Card], starter: Card) -> u8 {
    let mut cards = Vec::with_capacity(hand.len() + 1);
    cards.extend_from_slice(hand);
    cards.push(starter);

    let pairs = generate_pairs(&cards);

    let mut score = 0;

    score += count_fifteens(&pairs);
    score += count_pairs(&pairs);
    score += count_runs(&cards);

    score
}

fn generate_pairs(cards: &[Card]) -> Vec<[&Card; 2]> {
    let n = cards.len();
    let mut pairs = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in i + 1..n {
            pairs.push([cards.get(i).unwrap(), cards.get(j).unwrap()]);
        }
    }
    pairs
}

fn count_fifteens(pairs: &[[&Card; 2]]) -> u8 {
    let mut score = 0;
    for [a, b] in pairs {
        if a.value() + b.value() == 15 {
            score += 2;
        }
    }
    score
}

fn count_pairs(pairs: &[[&Card; 2]]) -> u8 {
    let mut score = 0;
    for [a, b] in pairs {
        if a.value() == b.value() {
            score += 2;
        }
    }
    score
}

fn count_run(cards: &[Card]) -> u8 {
    cards.sort();
    let lowest = cards[0].order();
    for i in 1..cards.len() {
        if cards[i].order() != (lowest + i as u8) {
            return 0;
        }
    }
    return cards.len() as u8;
}

fn generate_trips(cards: &[Card]) -> Vec<[&Card; 3]> {
    let n = cards.len();
    let mut trips = Vec::with_capacity(n * (n - 1) * (n - 2) / 6);
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                trips.push([
                    cards.get(i).unwrap(),
                    cards.get(j).unwrap(),
                    cards.get(k).unwrap(),
                ]);
            }
        }
    }
    trips
}

fn generate_quads(cards: &[Card]) -> Vec<([&Card; 4])> {
    let n = cards.len();
    let mut quads = Vec::with_capacity(n * (n - 1) * (n - 2) * (n - 3) / 24);
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    quads.push([
                        cards.get(i).unwrap(),
                        cards.get(j).unwrap(),
                        cards.get(k).unwrap(),
                        cards.get(l).unwrap(),
                    ]);
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
        let cards = vec![
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
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Five,
            },
        ];
        let pairs = generate_pairs(&cards);
        let score = count_fifteens(&pairs);
        assert_eq!(score, 4);
    }
}
