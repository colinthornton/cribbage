use core::time;
use std::thread;

use card::{Card, Deck};
use itertools::Itertools;
use the_play::score_the_play;
use the_show::score_the_show;

mod card;
mod the_play;
mod the_show;

fn main() {
    let mut p1_score = 0u8;
    let mut p2_score = 0u8;

    loop {
        let mut deck = Deck::new();

        let mut p1_hand = deck.draw_n(6).unwrap();
        let mut p2_hand = deck.draw_n(6).unwrap();

        let mut crib = Vec::new();
        crib.push(p1_hand.pop().unwrap());
        crib.push(p1_hand.pop().unwrap());
        crib.push(p2_hand.pop().unwrap());
        crib.push(p2_hand.pop().unwrap());

        let starter = deck.draw().unwrap();
        // his heels

        let mut count = 0u8;
        let mut played_cards = Vec::new();
        let mut p1_go = false;
        let mut p2_go = false;
        let mut p1_play_hand = p1_hand.clone();
        let mut p2_play_hand = p2_hand.clone();

        println!("\nThe Play");
        loop {
            if p1_play_hand.is_empty() && p2_play_hand.is_empty() {
                break;
            }

            let p1_playable = playable_card_indexes(&p1_play_hand, count);
            if p1_playable.len() > 0 {
                let i = p1_playable[0];
                let card = p1_play_hand[i];
                p1_play_hand.remove(i);
                played_cards.push(card);
                count += card.count_value();
                display_play("P1", card, count);

                let score = score_the_play(&played_cards);
                if score > 0 {
                    p1_score += score;
                    display_score(p1_score, p2_score)
                }
            } else if p2_go {
                if count == 31 {
                    p2_score += 2;
                } else {
                    p2_score += 1;
                }
                display_score(p1_score, p2_score);

                count = 0;
                p1_go = false;
                p2_go = false;
                continue;
            } else {
                p1_go = true;
            }

            let p2_playable = playable_card_indexes(&p2_play_hand, count);
            if p2_playable.len() > 0 {
                let i = p2_playable[0];
                let card = p2_play_hand[i];
                p2_play_hand.remove(i);
                played_cards.push(card);
                count += card.count_value();
                display_play("P2", card, count);

                let score = score_the_play(&played_cards);
                if score > 0 {
                    p2_score += score;
                    display_score(p1_score, p2_score)
                }
            } else if p1_go {
                if count == 31 {
                    p2_score += 2;
                } else {
                    p2_score += 1;
                }
                display_score(p1_score, p2_score);

                count = 0;
                p1_go = false;
                p2_go = false;
                continue;
            } else {
                p2_go = true;
            }
        }

        println!("\nThe Show");

        display_show("P1", &p1_hand, &starter);
        let score = score_the_show(&p1_hand, &starter);
        p1_score += score;
        display_score(p1_score, p2_score);

        display_show("P2", &p2_hand, &starter);
        let score = score_the_show(&p2_hand, &starter);
        p2_score += score;
        display_score(p1_score, p2_score);

        if p1_score >= 121 || p2_score >= 121 {
            break;
        }
    }
}

fn display_play(player: &str, card: Card, count: u8) {
    println!("{player}: {card} {count}");
    sleep();
}

fn display_show(player: &str, hand: &[Card], starter: &Card) {
    println!(
        "{player}: {} - {} {} {} {}",
        starter, hand[0], hand[1], hand[2], hand[3]
    );
    sleep();
}

fn display_score(p1: u8, p2: u8) {
    println!("Score P1: {:03} P2: {:03}", p1, p2);
    sleep();
}

fn sleep() {
    thread::sleep(time::Duration::from_millis(1000));
}

fn playable_card_indexes(hand: &[Card], count: u8) -> Vec<usize> {
    hand.iter()
        .enumerate()
        .filter(|(_i, card)| card.count_value() < 31 - count)
        .map(|(i, _)| i)
        .collect_vec()
}
