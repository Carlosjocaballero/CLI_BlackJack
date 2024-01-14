use std::collections::btree_map::RangeMut;
use std::io::{self, Write};
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
            
            // Reset points for house and user
            self.house.curr_points = 0;
            self.user.curr_points = 0;

            // House draws 2 cards
            self.house.cards[0] = self.draw_card();
            self.house.cards[1] = self.draw_card();

            // House stores the addition of card values
            self.house.curr_points += self.house.cards[0].value;
            self.house.curr_points += self.house.cards[0].value;


            // User draws 2 cards
            // self.user.cards[0] = self.draw_card();
            // self.user.cards[1] = self.draw_card();

            // ---- TESTING ACE ----
            self.user.cards[0] = Card::new('♥', 10);
            self.user.cards[1] = Card::new('A', 1);

            // User stores the addition of card values
            self.user.curr_points += self.user.cards[0].value;
            self.user.curr_points += self.user.cards[1].value;
            
            // Print playing cards
            self.print_playing_cards_hidden();

            self.evaluate_ace();
            
            if self.user.curr_points == 21 || self.check_user_blackjack() {
                self.control.game_state = GameState::Blackjack;
            } else{
                self.control.game_state = GameState::Choosing;
            }

            match self.control.game_state {
                GameState::Blackjack => {
                        println!("BlackJack!");
                        // continue;
                },
                GameState::Ace => {

                },
                _ => ()
            }


            println!("You have {}. What would you want to do?  1.Hit 2.Stand", self.user.curr_points);

            self.control.game_state = GameState::End;
        }

        
    }

    fn draw_card(&mut self) -> Card {
        let random = self.cards.deck.choose(&mut rand::thread_rng()).unwrap().clone();
        random
    }

    fn print_playing_cards_hidden(&mut self) {
        println!("\n------ HOUSE ------");
        for i in 0..self.house.cards.len() - 1 {
            print!("Card #{}: \n", i+1);
            self.house.cards[i].print_card();
        }
        print!("Card #2: \n");
        self.house.cards[1].print_blank_card();

        println!("\n------ USER ------");
        for i in 0..self.user.cards.len() {
            print!("Card #{}: \n", i+1);
            self.user.cards[i].print_card();
        }
    }

    fn check_user_blackjack(&mut self) -> bool{
        (self.user.cards[0].value == 1 && self.user.cards[1].value == 10) || (self.user.cards[0].value == 10 && self.user.cards[1].value == 1) 
    }

    fn evaluate_ace(&mut self) {
        for i in 0..self.user.cards.len() {
            if self.user.cards[i].value == 1 {
                self.user.has_ace = true;
                self.user.ace_pos = i;
            }
        }
    }
}