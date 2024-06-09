use crate::card::{rank_from_run_order, Card, Rank, Suit};
use crate::game::{GameAction, GameEvent};
use crate::the_play::score_the_play;
use itertools::Itertools;
use std::sync::mpsc::{Receiver, SyncSender};
use std::{fmt, thread, time};
use strum::{EnumCount, IntoEnumIterator};

struct Combo {
    kind: ComboKind,
    cards: Vec<Card>,
    score: f32,
}

enum ComboKind {
    Fifteen,
    PotentialFifteen,
    Pair,
    Run,
    PotentialRun,
    Flush,
    PotentialFlush,
    PotentialNobs,
}

impl fmt::Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{} ", card)?;
        }
        write!(f, "- {} for {}", &self.kind, &self.score)
    }
}

impl fmt::Display for ComboKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ComboKind::Fifteen => write!(f, "fifteen"),
            ComboKind::PotentialFifteen => write!(f, "potential fifteen"),
            ComboKind::Pair => write!(f, "pair"),
            ComboKind::Run => write!(f, "run"),
            ComboKind::PotentialRun => write!(f, "potential run"),
            ComboKind::Flush => write!(f, "flush"),
            ComboKind::PotentialFlush => write!(f, "potential flush"),
            ComboKind::PotentialNobs => write!(f, "potential nobs"),
        }
    }
}

pub fn launch_ai(event_receiver: Receiver<GameEvent>, action_sender: SyncSender<GameAction>) {
    loop {
        let event = event_receiver.recv();
        thread::sleep(time::Duration::from_millis(1000));
        match event {
            Ok(GameEvent::Deal { cards, dealer }) => {
                let discarded = discard_cards(cards, dealer);
                let Ok(_) = action_sender.send(GameAction::Discard { discarded }) else {
                    break;
                };
            }
            Ok(GameEvent::PlayRequest {
                hand,
                played,
                count,
            }) => {
                let card = select_play(hand, played, count);
                let Ok(_) = action_sender.send(GameAction::Play { card }) else {
                    break;
                };
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn discard_cards(cards: Vec<Card>, dealer: bool) -> [Card; 2] {
    let mut deck = Vec::with_capacity(Rank::COUNT * Suit::COUNT - cards.len());
    for rank in Rank::iter() {
        for suit in Suit::iter() {
            let card = Card::new(suit, rank);
            if !cards.contains(&card) {
                deck.push(card);
            }
        }
    }

    let mut cards = cards.to_owned();
    cards.sort_by(|a, b| a.run_cmp(b));

    let hands = cards.clone().into_iter().combinations(4).collect_vec();
    let discards = cards
        .into_iter()
        .combinations(2)
        .collect_vec()
        .into_iter()
        .rev()
        .collect_vec();

    // get the score for every combination of hand/discard
    let mut results = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let discarded = &discards[i];
            let mut combos: Vec<Combo> = Vec::new();
            let score = score(&hand, &discarded, &deck, dealer, &mut combos);
            let count_total = count_total(&hand);
            (hand, discarded, score, count_total, combos)
        })
        .collect_vec();

    // choose highest scoring hand with preference to smaller count totals
    results.sort_by(|(_, _, _, count_total_a, _), (_, _, _, count_total_b, _)| {
        count_total_b.cmp(count_total_a)
    });
    results.sort_by(|(_, _, score_a, _, _), (_, _, score_b, _, _)| {
        score_b.partial_cmp(score_a).unwrap()
    });

    // Debugging
    // let result = &results[0];
    // let hand = &result.0;
    // let discarded = &result.1;
    // let score = &result.2;
    // let dealer_msg = if dealer { "(dealer)" } else { "" };
    // println!(
    //     "{} {} {} {} - {} {} for {} {}",
    //     hand[0], hand[1], hand[2], hand[3], discarded[0], discarded[1], score, dealer_msg
    // );

    // let combos = &result.4;
    // for combo in combos {
    //     println!("{}", combo);
    // }

    let discarded = results[0].1;
    [discarded[0], discarded[1]]
}

fn score(
    hand: &[Card],
    discarded: &[Card],
    deck: &[Card],
    dealer: bool,
    combos: &mut Vec<Combo>,
) -> f32 {
    let hand_score = count_fifteens(hand, deck, combos)
        + count_pairs(hand, deck, combos)
        + count_runs(hand, deck, combos)
        + count_flush(hand, deck, combos)
        + count_nobs(hand, deck, combos);

    let discard_score = count_fifteens(discarded, deck, combos)
        + count_pairs(discarded, deck, combos)
        + count_runs(discarded, deck, combos)
        + count_flush(discarded, deck, combos)
        + count_nobs(discarded, deck, combos);

    if dealer {
        hand_score + discard_score
    } else {
        hand_score - discard_score
    }
}

fn count_total(hand: &[Card]) -> u8 {
    hand.iter().map(|card| card.count_value()).sum()
}

fn count_fifteens(cards: &[Card], deck: &[Card], combos: &mut Vec<Combo>) -> f32 {
    let card_combinations = (1..=cards.len())
        .map(|size| cards.iter().combinations(size))
        .into_iter()
        .flatten();

    let score = card_combinations
        .map(|cards| {
            let count: u8 = cards.iter().map(|card| card.count_value()).sum();
            let score: f32;
            if count == 15 {
                score = 2f32;
                combos.push(Combo {
                    kind: ComboKind::Fifteen,
                    cards: cards.iter().map(|card| **card).collect_vec(),
                    score,
                });
            } else if (5..=15).contains(&count) {
                score = potential_score(filter_by_count(deck, 15 - count).len(), deck.len(), 2);
                combos.push(Combo {
                    kind: ComboKind::PotentialFifteen,
                    cards: cards.iter().map(|card| **card).collect_vec(),
                    score,
                });
            } else {
                score = 0f32;
            }
            score
        })
        .sum();
    score
}

fn count_pairs(cards: &[Card], deck: &[Card], combos: &mut Vec<Combo>) -> f32 {
    let card_combinations = cards.iter().combinations(2);

    let score = card_combinations
        .map(|cards| {
            let rank = cards[0].rank();
            if cards[1].rank() == rank {
                let score = 2f32 + potential_score(filter_by_rank(deck, rank).len(), deck.len(), 4);
                combos.push(Combo {
                    kind: ComboKind::Pair,
                    cards: cards.iter().map(|card| **card).collect_vec(),
                    score,
                });
                score
            } else {
                0f32
            }
        })
        .sum();
    score
}

fn count_runs(cards: &[Card], deck: &[Card], combos: &mut Vec<Combo>) -> f32 {
    for size in (2..=cards.len()).rev() {
        let score = cards
            .iter()
            .combinations(size)
            .map(|cards| count_run(&cards, deck, combos))
            .sum();
        if score != 0f32 {
            return score;
        }
    }
    0f32
}

fn count_flush(hand: &[Card], deck: &[Card], combos: &mut Vec<Combo>) -> f32 {
    let suit = hand[0].suit();
    if !hand[1..].iter().all(|card| card.suit() == suit) {
        return 0f32;
    }

    let remaining_of_suit = deck
        .iter()
        .filter(|card| card.suit() == suit)
        .collect_vec()
        .len();

    // Small chance to get the other two cards flushed in the crib
    if hand.len() == 2 {
        let score = (remaining_of_suit as f32 / deck.len() as f32)
            * (remaining_of_suit as f32 - 1f32)
            / (deck.len() as f32 - 1f32)
            * 4f32;
        combos.push(Combo {
            kind: ComboKind::PotentialFlush,
            cards: hand.to_vec(),
            score,
        });
        return score;
    }

    let score = 4f32 + potential_score(remaining_of_suit, deck.len(), 1);
    combos.push(Combo {
        kind: ComboKind::Flush,
        cards: hand.to_vec(),
        score,
    });
    score
}

fn count_nobs(hand: &[Card], deck: &[Card], combos: &mut Vec<Combo>) -> f32 {
    return hand
        .iter()
        .map(|card| {
            if card.rank() != Rank::Jack {
                return 0f32;
            }

            let suit = card.suit();
            let remaining_of_suit = deck
                .iter()
                .filter(|card| card.suit() == suit)
                .collect_vec()
                .len();
            let score = potential_score(remaining_of_suit, deck.len(), 1);
            combos.push(Combo {
                kind: ComboKind::PotentialNobs,
                cards: vec![*card],
                score,
            });
            score
        })
        .sum();
}

fn count_run(cards: &[&Card], deck: &[Card], combos: &mut Vec<Combo>) -> f32 {
    let n = cards.len() as u8;
    let start = cards[0].run_order();

    // bail early if any cards are off by more than 1
    let mut offset = false;
    if cards.iter().enumerate().any(|(i, card)| {
        let expected = start + i as u8;
        let actual = card.run_order();
        if actual < expected {
            return true;
        }
        match actual - expected {
            1 => {
                offset = true;
                false
            }
            0 => {
                if offset {
                    true
                } else {
                    false
                }
            }
            _ => true,
        }
    }) {
        return 0f32;
    }

    let actual_ranks = cards.iter().map(|card| card.rank()).collect_vec();
    let expected_ranks = (start..start + n)
        .map(|run_order| rank_from_run_order(run_order).unwrap())
        .collect_vec();
    let missing_ranks = expected_ranks
        .iter()
        .filter(|rank| !actual_ranks.contains(rank))
        .collect_vec();

    if missing_ranks.len() == 0 {
        let end = cards.last().unwrap().run_order();

        if n == 2 {
            // potential run
            let score;
            if cards[0].rank() == Rank::Ace {
                score = potential_score(
                    filter_by_rank(deck, rank_from_run_order(end + 1).unwrap()).len(),
                    deck.len(),
                    3,
                );
            } else if cards.last().unwrap().rank() == Rank::King {
                score = potential_score(
                    filter_by_rank(deck, rank_from_run_order(start - 1).unwrap()).len(),
                    deck.len(),
                    3,
                );
            } else {
                score = potential_score(
                    filter_by_rank(deck, rank_from_run_order(start - 1).unwrap()).len()
                        + filter_by_rank(deck, rank_from_run_order(end + 1).unwrap()).len(),
                    deck.len(),
                    3,
                );
            };

            combos.push(Combo {
                kind: ComboKind::PotentialRun,
                cards: cards.iter().map(|card| **card).collect_vec(),
                score,
            });
            return score;
        }

        // complete run
        // we score n guaranteed, a card on either side nets another point, a card within the run scores another n points
        let same_rank = cards
            .iter()
            .map(|card| filter_by_rank(deck, card.rank()).len())
            .sum();
        let score;
        if cards[0].rank() == Rank::Ace {
            score = n as f32
                + potential_score(same_rank, deck.len(), n)
                + potential_score(
                    filter_by_rank(deck, rank_from_run_order(end + 1).unwrap()).len(),
                    deck.len(),
                    1,
                );
        } else if cards.last().unwrap().rank() == Rank::King {
            score = n as f32
                + potential_score(same_rank, deck.len(), n)
                + potential_score(
                    filter_by_rank(deck, rank_from_run_order(start - 1).unwrap()).len(),
                    deck.len(),
                    1,
                );
        } else {
            score = n as f32
                + potential_score(same_rank, deck.len(), n)
                + potential_score(
                    filter_by_rank(deck, rank_from_run_order(start - 1).unwrap()).len(),
                    deck.len(),
                    1,
                )
                + potential_score(
                    filter_by_rank(deck, rank_from_run_order(end + 1).unwrap()).len(),
                    deck.len(),
                    1,
                );
        };

        combos.push(Combo {
            kind: ComboKind::Run,
            cards: cards.iter().map(|card| **card).collect_vec(),
            score,
        });
        return score;
    } else if missing_ranks.len() == 1 {
        let score = potential_score(filter_by_rank(deck, *missing_ranks[0]).len(), deck.len(), n);
        combos.push(Combo {
            kind: ComboKind::PotentialRun,
            cards: cards.iter().map(|card| **card).collect_vec(),
            score,
        });
        return score;
    }
    0f32
}

fn potential_score(n_card_needed: usize, n_card_remaining: usize, potential_score: u8) -> f32 {
    n_card_needed as f32 / n_card_remaining as f32 * potential_score as f32
}

fn select_play(hand: Vec<Card>, played: Vec<Card>, count: u8) -> Card {
    let playable_cards = hand
        .into_iter()
        .filter(|card| card.count_value() + count <= 31)
        .collect_vec();
    let mut results = playable_cards
        .iter()
        .map(|card| {
            let mut cards = played.to_owned();
            cards.push(card.to_owned());
            let score = score_the_play(&cards);

            if score == 0 {
                let Some(last_card) = played.last() else {
                    return (card, 0i8);
                };

                // discourage giving the oppontent a run opportunity
                let last_run_order = last_card.run_order();
                let card_run_order = card.run_order();
                let diff = last_run_order.max(card_run_order) - last_run_order.min(card_run_order);
                if diff <= 2 {
                    return (card, -1);
                }

                let count = card.count_value() + count;
                // discourage making the count 10 since holding 5s is common
                if count == 10 {
                    return (card, -1);
                }

                // encourage keeping the count 11+ away from a score
                if count < 5 || count < 21 {
                    return (card, 1);
                }
            }

            return (card, score as i8);
        })
        .collect_vec();

    // sort by count value descending, then score value descending
    results.sort_by(|(a, _), (b, _)| b.count_value().cmp(&a.count_value()));
    results.sort_by(|(_, a), (_, b)| b.cmp(a));

    results[0].0.to_owned()
}

fn filter_by_count(deck: &[Card], count: u8) -> Vec<&Card> {
    deck.iter()
        .filter(|card| card.count_value() == count)
        .collect_vec()
}

fn filter_by_rank(deck: &[Card], rank: Rank) -> Vec<&Card> {
    deck.iter().filter(|card| card.rank() == rank).collect_vec()
}
