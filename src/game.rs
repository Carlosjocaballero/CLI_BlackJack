use std::io;

use crate::user::User;
use crate::house::House;
use crate::control::*;
use crate::cards::Cards;



pub struct Game {
    pub user : User,
    pub house: House,
    pub control: Control,
    pub cards: Cards
}

impl Game{
    pub fn new() -> Self {
        Game {
            user: User::new(),
            house: House::new(),
            control: Control::new(),
            cards: Cards::new()
        }
    }

    pub fn init(&mut self){
        while self.control.game_state != GameState::End{
            println!("The house draws");
            self.house_draw();
            self.control.game_state = GameState::End;
        }
        
    }

    fn house_draw(&mut self){
        println!("{}", self.cards.stack.len());
    }
}