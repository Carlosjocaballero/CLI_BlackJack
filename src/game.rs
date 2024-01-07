use std::collections::btree_map::RangeMut;
use std::io;
use rand::Rng;
use rand::prelude::SliceRandom;

use crate::user::User;
use crate::house::House;
use crate::control::*;
use crate::cards::Cards;
use crate::card::Card;



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
            println!("The house draws ");
            self.house.first_card = self.draw_card();
            self.house.second_card = self.draw_card();

            println!("");
            print!("HOUSE First card: \n");
            self.house.first_card.print_card();
            print!("HOUSE Second card: \n");
            self.house.second_card.print_blank_card();


            self.control.game_state = GameState::End;
        }
        
    }

    fn draw_card(&mut self) -> Card {
        let random = self.cards.deck.choose(&mut rand::thread_rng()).unwrap().clone();
        random
    }
}