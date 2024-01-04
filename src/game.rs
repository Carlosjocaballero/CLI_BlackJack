use crate::user::User;
use crate::house::House;
use crate::control::*;



pub struct Game {
    pub user : User,
    pub house: House,
    pub control: Control
}

impl Game{
    pub fn new() -> Self {
        Game {
            user: User::new(),
            house: House::new(),
            control: Control::new()
        }
    }

    pub fn init(&mut self) -> bool{
        while self.control.game_state != GameState::End{
            println!("hello!");
            self.control.game_state = GameState::End;
        }
        true
    }
}