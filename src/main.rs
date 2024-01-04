use crate::game::Game;

mod game;
pub mod user;
pub mod house;

fn main() {
    let mut game : Game = Game::new();
    // println!("{}", game.curr_score);
    // game.curr_score = 5;
    // println!("{}", game.curr_score);
}
