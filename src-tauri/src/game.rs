use crate::{
    card::{Card, Deck, Rank},
    the_play::score_the_play,
    the_show::score_the_show,
};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::{
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread, time,
};

const PLAYERS_SIZE: usize = 2;
const CRIB_SIZE: usize = 4;
const PLAYED_SIZE: usize = 8;
const MAX_SCORE: u8 = 121;
const MAX_COUNT: u8 = 31;

pub struct Game {
    players: Vec<Player>,
    dealer_index: usize,
    player_index: usize,
    deck: Deck,
    crib: Vec<Card>,
    starter: Option<Card>,
    played: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::with_capacity(PLAYERS_SIZE),
            dealer_index: 0,
            player_index: 1,
            deck: Deck::new(),
            crib: Vec::with_capacity(CRIB_SIZE),
            starter: None,
            played: Vec::with_capacity(PLAYED_SIZE),
        }
    }

    pub fn register_player(
        &mut self,
        id: String,
        event_sender: SyncSender<GameEvent>,
    ) -> SyncSender<GameAction> {
        if self.players.len() == 2 {
            panic!("Can't register more than two players");
        }

        let (action_sender, action_receiver) = sync_channel(1);
        self.players
            .push(Player::new(id, event_sender, action_receiver));
        println!("{} joins", self.players.last().unwrap().id);
        action_sender
    }

    pub fn start(&mut self) {
        if self.players.len() < PLAYERS_SIZE {
            panic!("Can't start")
        }

        self.dealer_index = thread_rng().gen_range(0..self.players.len());
        self.player_index = (self.dealer_index + 1) % self.players.len();
        println!("{} gets the first deal", self.dealer().id);

        self.game_loop()
    }

    fn game_loop(&mut self) {
        loop {
            // Deal
            self.deal();

            for (i, player) in self.players.iter_mut().enumerate() {
                player.send_event(GameEvent::Deal {
                    cards: player.hand.to_owned(),
                    dealer: i == self.dealer_index,
                });
                let discarded = player.await_discard();
                self.crib.extend(discarded);
            }
            println!("{} deals", self.dealer().id);

            // Discard
            for player in self.players.iter_mut() {
                // let discarded = player.await_discard();
                // self.crib.extend(discarded);
            }

            // Cut
            let starter = self.deck.draw().unwrap();
            self.starter = Some(starter);
            println!("{} cuts {}", self.player().id, starter);
            if starter.rank() == Rank::Jack {
                println!("{}: 2 for his heels", self.dealer().id);
                let game_over = self.add_score(self.dealer_index, 2);
                if game_over {
                    return;
                }
            }

            // Play
            while !self.players.iter().all(|player| player.played_out()) {
                if self.player().can_play(self.count()) {
                    self.player().send_event(GameEvent::PlayRequest {
                        hand: self.player().unplayed_cards().collect_vec().to_owned(),
                        played: self.played.to_owned(),
                        count: self.count(),
                    });
                    let count = self.count();
                    let card = self.player_mut().await_play(count);
                    self.played.push(card);

                    let score = score_the_play(&self.played);
                    let mut score_msg = "".to_string();
                    if score > 0 {
                        score_msg = format!(" for {}", score);
                    }
                    println!(
                        "{}: {} {}{}",
                        self.player().id,
                        card,
                        self.count(),
                        score_msg
                    );

                    let game_over = self.add_score(self.player_index, score);
                    if game_over {
                        return;
                    }

                    if self.count() == MAX_COUNT
                        || self.players.iter().all(|player| player.played_out())
                    {
                        let game_over;
                        if self.count() == MAX_COUNT {
                            println!("{}: {} for 2", self.player().id, MAX_COUNT);
                            game_over = self.add_score(self.player_index, 2);
                        } else {
                            println!("{}: 1 for last card", self.player().id);
                            game_over = self.add_score(self.player_index, 1);
                        }
                        if game_over {
                            return;
                        }

                        self.played = Vec::with_capacity(PLAYED_SIZE);
                        for player in self.players.iter_mut() {
                            player.go = false;
                        }
                    }
                } else if self.next_player().go {
                    println!("{}: 1 for the go", self.player().id);
                    let game_over = self.add_score(self.player_index, 1);
                    if game_over {
                        return;
                    }

                    self.played = Vec::with_capacity(PLAYED_SIZE);
                    for player in self.players.iter_mut() {
                        player.go = false;
                    }
                } else if !self.player().go {
                    self.player_mut().go = true;
                    println!("{}: go", self.player().id);
                }

                self.switch_player();
            }

            // Show
            let shower_i = (self.dealer_index + 1) % 2;
            let shower = &self.players[shower_i];
            let score = score_the_show(&shower.hand, &self.starter.unwrap());
            println!(
                "{} hand: {} - {} {} {} {} for {}",
                shower.id,
                self.starter.unwrap(),
                shower.hand[0],
                shower.hand[1],
                shower.hand[2],
                shower.hand[3],
                score,
            );
            let game_over = self.add_score(shower_i, score);
            if game_over {
                return;
            }

            thread::sleep(time::Duration::from_secs(2));

            let score = score_the_show(&self.dealer().hand, &self.starter.unwrap());
            println!(
                "{} hand: {} - {} {} {} {} for {}",
                self.dealer().id,
                self.starter.unwrap(),
                self.dealer().hand[0],
                self.dealer().hand[1],
                self.dealer().hand[2],
                self.dealer().hand[3],
                score,
            );
            let game_over = self.add_score(self.dealer_index, score);
            if game_over {
                return;
            }

            thread::sleep(time::Duration::from_secs(2));

            let score = score_the_show(&self.crib, &self.starter.unwrap());
            println!(
                "{} crib: {} - {} {} {} {} for {}",
                self.dealer().id,
                self.starter.unwrap(),
                self.crib[0],
                self.crib[1],
                self.crib[2],
                self.crib[3],
                score,
            );
            let game_over = self.add_score(self.dealer_index, score);
            if game_over {
                return;
            }

            thread::sleep(time::Duration::from_secs(2));

            // Cleanup
            self.dealer_index = (self.dealer_index + 1) % 2;
            self.player_index = (self.dealer_index + 1) % 2;
            self.deck = Deck::new();
            self.crib = Vec::with_capacity(CRIB_SIZE);
            self.starter = None;
            self.played = Vec::with_capacity(PLAYED_SIZE);
            for player in self.players.iter_mut() {
                player.go = false;
            }
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

    fn next_player(&self) -> &Player {
        &self.players[(self.player_index + 1) % 2]
    }

    fn deal(&mut self) {
        for player in self.players.iter_mut() {
            let cards = self.deck.draw_n(6).unwrap();
            player.set_hand(&cards);
        }
    }

    /// Returns true if game is over
    fn add_score(&mut self, player_index: usize, score: u8) -> bool {
        if score == 0 {
            return false;
        }

        let new_score = {
            let player = &mut self.players[player_index];
            player.add_score(score)
        };

        println!(
            "SCORE {}: {} {}: {}",
            self.players[0].id, self.players[0].score, self.players[1].id, self.players[1].score
        );

        if new_score == MAX_SCORE {
            println!("{} wins", self.players[player_index].id);
            return true;
        }
        return false;
    }

    fn switch_player(&mut self) {
        self.player_index = (self.player_index + 1) % PLAYERS_SIZE;
    }

    fn count(&self) -> u8 {
        self.played.iter().map(|card| card.count_value()).sum()
    }
}

struct Player {
    id: String,
    event_sender: SyncSender<GameEvent>,
    action_receiver: Receiver<GameAction>,
    score: u8,
    hand: Vec<Card>,
    played: Vec<Card>,
    go: bool,
}

impl Player {
    fn new(
        id: String,
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
            go: false,
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

    fn await_play(&mut self, count: u8) -> Card {
        loop {
            let action = self.await_action();
            match action {
                GameAction::Play { card } => {
                    if !self.playable_cards(count).contains(&card) {
                        continue;
                    }
                    self.played.push(card);
                    return card;
                }
                _ => continue,
            }
        }
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
