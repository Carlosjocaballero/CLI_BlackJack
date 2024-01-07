use std::env;

use crate::game::Game;


mod game;
pub mod user;
pub mod house;
pub mod control;
pub mod cards;
pub mod card;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut game : Game = Game::new();
    println!("Start the game!");
    game.init();
    println!("Goodbye!");
    
}
