// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ai::launch_ai;
use game::Game;
use std::{
    sync::{mpsc::sync_channel, Mutex},
    thread,
};
use tauri::State;

mod ai;
mod card;
mod game;
mod the_play;
mod the_show;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Game::new()))
        .invoke_handler(tauri::generate_handler![start_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn start_game(game: State<'_, Mutex<Game>>) -> Result<(), ()> {
    let mut game = game.lock().unwrap();

    {
        let (event_sender, event_receiver) = sync_channel(1);
        let action_sender = game.register_player("CPU".into(), event_sender);
        thread::spawn(move || launch_ai(event_receiver, action_sender));
    }

    {
        let (event_sender, event_receiver) = sync_channel(1);
        let action_sender = game.register_player("T-800".into(), event_sender);
        thread::spawn(move || launch_ai(event_receiver, action_sender));
    }

    game.start();
    Ok(())
}
