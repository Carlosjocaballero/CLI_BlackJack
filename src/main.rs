use crate::game::Game;

mod game;
pub mod user;
pub mod house;
pub mod control;

fn main() {
    let mut game : Game = Game::new();
    println!("Start the game!");
    game.init();
    println!("Goodbye!");
    
}
