use crate::card::{Card, Rank};
use crate::game::{GameAction, GameEvent};
use itertools::Itertools;
use std::sync::mpsc::{Receiver, SyncSender};

pub fn launch_ai(event_receiver: Receiver<GameEvent>, action_sender: SyncSender<GameAction>) {
    loop {
        let event = event_receiver.recv();
        match event {
            Ok(GameEvent::Deal { cards, dealer }) => {
                println!(
                    "Deal {{ cards: {}, dealer: {} }}",
                    cards
                        .iter()
                        .map(|card| card.to_string())
                        .collect_vec()
                        .join(", "),
                    dealer
                );
                let discarded = discard_cards(cards, dealer);
                println!(
                    "discarded: {}",
                    discarded
                        .iter()
                        .map(|card| card.to_string())
                        .collect_vec()
                        .join(", ")
                );
                let Ok(_) = action_sender.send(GameAction::Discard { discarded }) else {
                    break;
                };
            }
            Ok(GameEvent::PlayRequest {
                hand,
                played,
                count,
            }) => {
                // TODO: play decision
                let card = hand
                    .into_iter()
                    .filter(|card| card.count_value() + count <= 31)
                    .collect_vec()[0];
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
            let score = score(&hand, &discarded, dealer);
            let count_total = count_total(&hand);
            (discarded, score, count_total)
        })
        .collect_vec();

    // choose highest scoring hand with preference to smaller count totals
    results
        .sort_by(|(_, _, count_total_a), (_, _, count_total_b)| count_total_b.cmp(count_total_a));
    results.sort_by(|(_, score_a, _), (_, score_b, _)| score_b.partial_cmp(score_a).unwrap());

    let discarded = results[0].0;
    [discarded[0], discarded[1]]
}

fn score(hand: &[Card], discarded: &[Card], dealer: bool) -> f32 {
    let mut score = 0f32;
    score += count_fifteens(hand);
    score += count_pairs(hand);
    score += count_runs(hand);
    score += count_flush(hand);
    score += count_nobs(hand);

    let mut discard_score = 0f32;
    discard_score += count_fifteens(discarded);
    discard_score += count_pairs(discarded);
    discard_score += count_runs(discarded);
    discard_score += count_flush(discarded);
    discard_score += count_nobs(discarded);

    if dealer {
        score + discard_score
    } else {
        score - discard_score
    }
}

fn count_total(hand: &[Card]) -> u8 {
    hand.iter().map(|card| card.count_value()).sum()
}

fn count_fifteens(cards: &[Card]) -> f32 {
    let card_combinations = (1..=cards.len())
        .map(|size| cards.iter().combinations(size))
        .into_iter()
        .flatten();

    let counts =
        card_combinations.map(|cards| cards.iter().map(|card| card.count_value()).sum::<u8>());

    let score = counts
        .map(|count| match count {
            15 => 2f32,
            6..=10 => potential_score(1, 2),
            5 => potential_score(4, 2),
            _ => 0f32,
        })
        .sum();
    score
}

fn count_pairs(cards: &[Card]) -> f32 {
    let card_combinations = cards.iter().combinations(2);

    let score = card_combinations
        .map(|cards| {
            if cards[0].rank() == cards[1].rank() {
                2f32
            } else {
                0f32
            }
        })
        .sum();
    score
}

fn count_runs(cards: &[Card]) -> f32 {
    for size in (2..=cards.len()).rev() {
        let score = cards
            .iter()
            .combinations(size)
            .map(|cards| count_run(&cards))
            .sum();
        if score != 0f32 {
            return score;
        }
    }
    0f32
}

fn count_flush(hand: &[Card]) -> f32 {
    let suit = hand[0].suit();
    if hand[1..].iter().all(|card| card.suit() == suit) {
        // Small chance to get the other two cards flushed in the crib
        if hand.len() == 2 {
            return 1f32 / 13f32 / 13f32 * 4f32;
        }
        return 4f32;
    }
    0f32
}

fn count_nobs(hand: &[Card]) -> f32 {
    for card in hand.iter() {
        if card.rank() == Rank::Jack {
            return 0.25;
        }
    }
    0f32
}

fn count_run(cards: &[&Card]) -> f32 {
    let n = cards.len() as u8;
    let start = cards[0].run_order();

    if n == 2 {
        let diff = ((start as i8) - (cards[1].run_order() as i8)).abs();
        match diff {
            // If two apart, roughly 1 in 13 chance for 3 points
            2 => return 1f32 / 13f32 * 3f32,
            // If one apart, roughly 2 in 13 chance for 3 points (but only 1 in 13 with Ace or King)
            1 => {
                let ranks = cards.iter().map(|card| card.rank()).collect_vec();
                if ranks.contains(&Rank::Ace) || ranks.contains(&Rank::King) {
                    return potential_score(1, 3);
                }
                return potential_score(2, 3);
            }
            _ => return 0f32,
        }
    }

    if cards
        .iter()
        .map(|card| card.run_order())
        .eq(start..start + n)
    {
        let ranks = cards.iter().map(|card| card.rank()).collect_vec();
        // we score n guaranteed, a card on either side nets another point, a card within the run scores another n points
        if ranks.contains(&Rank::Ace) || ranks.contains(&Rank::King) {
            return n as f32 + potential_score(1, 1) + potential_score(n, n);
        }
        return n as f32 + potential_score(2, 1) + potential_score(n, n);
    }

    0f32
}

fn potential_score(n_card_needed: u8, potential_score: u8) -> f32 {
    n_card_needed as f32 / 13f32 * potential_score as f32
}
