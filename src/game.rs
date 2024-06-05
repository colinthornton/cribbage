use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use crate::card::{Card, Deck};

pub struct Game {
    players: [Player; 2],
    dealer_index: usize,
    player_index: usize,
    deck: Deck,
    crib: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: [Player::new(sync_channel(1)), Player::new(sync_channel(1))],
            dealer_index: 0,
            player_index: 1,
            deck: Deck::new(),
            crib: Vec::new(),
        }
    }

    pub fn event_receivers(&self) -> []

    fn deal(&mut self) {
        for player in self.players.iter_mut() {
            player.hand = self.deck.draw_n(6).unwrap();
        }
    }

    fn cleanup(&mut self) {
        self.dealer_index = self.dealer_index + 1 % self.players.len();
        self.deck = Deck::new();
    }
}

pub struct Player {
    score: u8,
    hand: Vec<Card>,
    go: bool,
    event_channel: (SyncSender<GameEvent>, Receiver<GameEvent>),
}

impl Player {
    pub fn new(event_channel: (SyncSender<GameEvent>, Receiver<GameEvent>)) -> Player {
        Player {
            score: 0,
            hand: Vec::new(),
            go: false,
            event_channel,
        }
    }
}

enum GameEvent {
    Deal,
}

enum GameAction {
    Discard { cards: [Card; 2] },
    Play { card: Card },
}

struct GameLaunch {
    p1_event_receiver: Receiver<GameEvent>,
    p1_action_sender: SyncSender<GameAction>,
    p2_event_receiver: Receiver<GameEvent>,
    p2_action_sender: SyncSender<GameAction>,
}
