use crate::{
    card::{Card, Deck, Rank},
    the_play::score_the_play,
    the_show::score_the_show,
};
use itertools::Itertools;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

const PLAYERS_SIZE: usize = 2;
const CRIB_SIZE: usize = 4;
const PLAYED_SIZE: usize = 8;
const MAX_SCORE: u8 = 121;
const MAX_COUNT: u8 = 31;

pub struct Game {
    state: GameState,
    players: Vec<Player>,
    dealer_index: usize,
    player_index: usize,
    deck: Deck,
    crib: Vec<Card>,
    starter: Option<Card>,
    played: Vec<Card>,
    go: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::AwaitingPlayers,
            players: Vec::with_capacity(PLAYERS_SIZE),
            dealer_index: 0,
            player_index: 1,
            deck: Deck::new(),
            crib: Vec::with_capacity(CRIB_SIZE),
            starter: None,
            played: Vec::with_capacity(PLAYED_SIZE),
            go: false,
        }
    }

    pub fn register_player(
        &mut self,
        event_sender: SyncSender<GameEvent>,
    ) -> SyncSender<GameAction> {
        if self.state != GameState::AwaitingPlayers {
            panic!("Can't register more than two players");
        }

        let (action_sender, action_receiver) = sync_channel(1);
        self.players.push(Player::new(
            self.players.len() + 1,
            event_sender,
            action_receiver,
        ));
        println!("P{} joins", self.players.last().unwrap().id);
        action_sender
    }

    pub fn start(&mut self) {
        if self.players.len() < PLAYERS_SIZE {
            panic!("Can't start")
        }
        self.transition(GameState::Deal);
    }

    fn transition(&mut self, state: GameState) {
        self.state = state;
        self.game_loop();
    }

    fn game_loop(&mut self) {
        match &self.state {
            GameState::AwaitingPlayers => {
                panic!("Unexpected state");
            }
            GameState::Deal => {
                self.deal();
                for player in self.players.iter() {
                    player.send_event(GameEvent::Deal {
                        cards: player.hand.to_owned(),
                        dealer: player.id == self.dealer().id,
                    });
                }

                println!("P{} deals", self.dealer().id);
                self.transition(GameState::Discard);
            }
            GameState::Discard => {
                for player in self.players.iter_mut() {
                    let discarded = player.await_discard();
                    println!("P{} discards", player.id);
                    self.crib.extend(discarded);
                }

                self.transition(GameState::Cut);
            }
            GameState::Cut => {
                let starter = self.deck.draw().unwrap();
                self.starter = Some(starter);
                println!("P{} cuts {}", self.player().id, starter);
                if starter.rank() == Rank::Jack {
                    self.add_score(self.dealer_index, 2);
                    if self.state == GameState::Over {
                        return;
                    }
                }

                self.transition(GameState::Play);
            }
            GameState::Play => {
                let player = self.player();
                let id = player.id;

                if player.can_play(self.count()) {
                    player.send_event(GameEvent::PlayRequest {
                        hand: player.unplayed_cards().collect_vec().to_owned(),
                        played: self.played.to_owned(),
                        count: self.count(),
                    });
                    let card = player.await_play(self.count());
                    self.played.push(card);

                    let player = self.player_mut();
                    player.play_card(card);

                    println!("P{}: {} {}", id, card, self.count());

                    let score = score_the_play(&self.played);
                    if score > 0 {
                        self.add_score(self.player_index, score);
                        if self.state == GameState::Over {
                            return;
                        }
                    }
                } else if self.go {
                    if self.count() == MAX_COUNT {
                        self.add_score(self.player_index, 2);
                    } else {
                        self.add_score(self.player_index, 1);
                    }
                    if self.state == GameState::Over {
                        return;
                    }
                    self.played = Vec::with_capacity(8 - self.played.len());
                    self.go = false;
                } else {
                    self.go = true;
                    println!("P{}: go", id);
                }

                if self.players.iter().all(|player| player.played_out()) {
                    if self.count() == MAX_COUNT {
                        self.add_score(self.player_index, 2);
                    } else {
                        self.add_score(self.player_index, 1);
                    }
                    if self.state == GameState::Over {
                        return;
                    }

                    self.transition(GameState::Show);
                } else {
                    if self.players[(self.player_index + 1) % 2].can_play(self.count()) {
                        self.next_player();
                    }
                    self.game_loop();
                }
            }
            GameState::Show => {
                let shower_i = (self.dealer_index + 1) % 2;
                let shower = &self.players[shower_i];
                let score = score_the_show(&shower.hand, &self.starter.unwrap());
                println!(
                    "P{} hand: {} - {} {} {} {} {}",
                    shower.id,
                    self.starter.unwrap(),
                    shower.hand[0],
                    shower.hand[1],
                    shower.hand[2],
                    shower.hand[3],
                    score,
                );
                self.add_score(shower_i, score);
                if self.state == GameState::Over {
                    return;
                }

                let dealer = self.dealer();
                let id = dealer.id;
                let score = score_the_show(&dealer.hand, &self.starter.unwrap());
                println!(
                    "P{} hand: {} - {} {} {} {} {}",
                    id,
                    self.starter.unwrap(),
                    dealer.hand[0],
                    dealer.hand[1],
                    dealer.hand[2],
                    dealer.hand[3],
                    score,
                );
                self.add_score(self.dealer_index, score);
                if self.state == GameState::Over {
                    return;
                }

                let score = score_the_show(&self.crib, &self.starter.unwrap());
                println!(
                    "P{} crib: {} - {} {} {} {} {}",
                    id,
                    self.starter.unwrap(),
                    self.crib[0],
                    self.crib[1],
                    self.crib[2],
                    self.crib[3],
                    score,
                );
                self.add_score(self.dealer_index, score);
                if self.state == GameState::Over {
                    return;
                }

                self.transition(GameState::Cleanup)
            }
            GameState::Cleanup => {
                self.dealer_index = (self.dealer_index + 1) % 2;
                self.player_index = (self.dealer_index + 1) % 2;
                self.deck = Deck::new();
                self.crib = Vec::with_capacity(CRIB_SIZE);
                self.starter = None;
                self.played = Vec::with_capacity(PLAYED_SIZE);
                self.go = false;

                self.transition(GameState::Deal);
            }
            GameState::Over => {}
        }
    }

    fn dealer(&self) -> &Player {
        &self.players[self.dealer_index]
    }

    fn player(&self) -> &Player {
        &self.players[self.player_index]
    }

    fn player_mut(&mut self) -> &mut Player {
        &mut self.players[self.player_index]
    }

    fn deal(&mut self) {
        for player in self.players.iter_mut() {
            let cards = self.deck.draw_n(6).unwrap();
            player.set_hand(&cards);
        }
    }

    fn add_score(&mut self, player_index: usize, score: u8) {
        let player = &mut self.players[player_index];
        let id = player.id;
        let new_score = player.add_score(score);

        println!(
            "SCORE P1: {} P2: {}",
            self.players[0].score, self.players[1].score
        );

        if new_score == MAX_SCORE {
            println!("P{} wins", id);
            self.transition(GameState::Over)
        }
    }

    fn next_player(&mut self) {
        self.player_index = (self.player_index + 1) % PLAYERS_SIZE;
    }

    fn count(&self) -> u8 {
        self.played.iter().map(|card| card.count_value()).sum()
    }
}

#[derive(Debug, PartialEq)]
enum GameState {
    AwaitingPlayers,
    Deal,
    Discard,
    Cut,
    Play,
    Show,
    Cleanup,
    Over,
}

struct Player {
    id: usize,
    event_sender: SyncSender<GameEvent>,
    action_receiver: Receiver<GameAction>,
    score: u8,
    hand: Vec<Card>,
    played: Vec<Card>,
}

impl Player {
    fn new(
        id: usize,
        event_sender: SyncSender<GameEvent>,
        action_receiver: Receiver<GameAction>,
    ) -> Player {
        Player {
            id,
            event_sender,
            action_receiver,
            score: 0,
            hand: Vec::with_capacity(6),
            played: Vec::with_capacity(4),
        }
    }

    fn set_hand(&mut self, cards: &[Card]) {
        self.hand = cards.to_owned();
        self.played = Vec::with_capacity(4);
    }

    fn add_score(&mut self, score: u8) -> u8 {
        self.score = (self.score + score).min(MAX_SCORE);
        self.score
    }

    fn send_event(&self, event: GameEvent) {
        self.event_sender.send(event.to_owned()).unwrap();
    }

    fn await_action(&self) -> GameAction {
        self.action_receiver.recv().unwrap()
    }

    fn await_discard(&mut self) -> [Card; 2] {
        loop {
            let action = self.await_action();
            match action {
                GameAction::Discard { discarded } => {
                    let new_hand = self
                        .hand
                        .iter()
                        .filter(|card| !discarded.contains(card))
                        .map(|card| card.to_owned())
                        .collect_vec();
                    if new_hand.len() == 4 {
                        self.set_hand(&new_hand);
                        return discarded;
                    }
                    continue;
                }
                _ => continue,
            }
        }
    }

    fn can_play(&self, count: u8) -> bool {
        !self.playable_cards(count).collect_vec().is_empty()
    }

    fn playable_cards(&self, count: u8) -> impl Iterator<Item = Card> + '_ {
        let max = MAX_COUNT - count;
        self.unplayed_cards()
            .filter(move |card| card.count_value() <= max)
    }

    fn unplayed_cards(&self) -> impl Iterator<Item = Card> + '_ {
        self.hand
            .clone()
            .into_iter()
            .filter(|card| !self.played.contains(card))
    }

    fn played_out(&self) -> bool {
        self.played.len() == self.hand.len()
    }

    fn await_play(&self, count: u8) -> Card {
        loop {
            let action = self.await_action();
            match action {
                GameAction::Play { card } => {
                    if !self.playable_cards(count).contains(&card) {
                        continue;
                    }
                    return card;
                }
                _ => continue,
            }
        }
    }

    fn play_card(&mut self, card: Card) {
        self.played.push(card);
    }
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    Deal {
        cards: Vec<Card>,
        dealer: bool,
    },
    PlayRequest {
        hand: Vec<Card>,
        played: Vec<Card>,
        count: u8,
    },
}

pub enum GameAction {
    Discard { discarded: [Card; 2] },
    Play { card: Card },
}
