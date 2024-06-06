use ai::launch_ai;
use game::Game;
use std::{sync::mpsc::sync_channel, thread};

mod ai;
mod card;
mod game;
mod the_play;
mod the_show;

fn main() {
    let mut game = Game::new();

    for _ in 0..2 {
        let (event_sender, event_receiver) = sync_channel(1);
        let action_sender = game.register_player(event_sender);
        thread::spawn(move || launch_ai(event_receiver, action_sender));
    }

    game.start();
}
