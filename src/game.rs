use crate::user::User;
use crate::house::House;



pub struct Game {
    pub user : User,
    pub house: House
}

impl Game{
    pub fn new() -> Self {
        Game {
            user: User::new(),
            house: House::new()
        }
    }

    pub fn init(&mut self){
        
    }
}