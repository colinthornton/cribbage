use crate::{
    card::Card,
    game::{GameAction, GameEvent},
};
use inquire::{list_option::ListOption, validator::Validation, InquireError, MultiSelect, Select};
use itertools::Itertools;
use std::sync::mpsc::{Receiver, SyncSender};

pub fn launch_human(event_receiver: Receiver<GameEvent>, action_sender: SyncSender<GameAction>) {
    loop {
        let event = event_receiver.recv();
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
    let validator = |a: &[ListOption<&Card>]| {
        if a.len() != 2 {
            return Ok(Validation::Invalid("Select 2 cards".into()));
        }
        return Ok(Validation::Valid);
    };

    let whose_crib;
    if dealer {
        whose_crib = "your crib";
    } else {
        whose_crib = "their crib";
    }
    let answer = MultiSelect::new(
        &format!("Select 2 cards to discard to {}:", whose_crib),
        cards.clone(),
    )
    .with_validator(validator)
    .with_page_size(cards.len())
    .prompt();

    match answer {
        Ok(discarded) => [discarded[0], discarded[1]],
        Err(err) => match err {
            InquireError::OperationCanceled => discard_cards(cards, dealer),
            _ => panic!(),
        },
    }
}

fn select_play(hand: Vec<Card>, played: Vec<Card>, count: u8) -> Card {
    let hand_clone = hand.clone();

    let validator = move |a: &[ListOption<&Card>]| {
        if a.len() != 1 {
            return Ok(Validation::Invalid("Select one card".into()));
        }
        let card = a[0].value;

        let playable_cards = hand_clone
            .iter()
            .filter(|card| card.count_value() + count <= 31)
            .collect_vec();
        if !playable_cards.contains(&card) {
            return Ok(Validation::Invalid("That card can't be played".into()));
        }

        return Ok(Validation::Valid);
    };

    let answer = MultiSelect::new("Select a card to play:", hand.clone())
        .with_validator(validator)
        .with_page_size(hand.len())
        .prompt();

    match answer {
        Ok(card) => card[0],
        Err(err) => match err {
            InquireError::OperationCanceled => select_play(hand, played, count),
            _ => panic!(),
        },
    }
}
