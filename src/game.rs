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
            self.house.first_card = self.draw_card();
            self.house.second_card = self.draw_card();

            // House stores the addition of card values
            self.house.curr_points += self.house.first_card.value;
            self.house.curr_points += self.house.second_card.value;


            // User draws 2 cards
            self.user.first_card = self.draw_card();
            self.user.second_card = self.draw_card();

            // User stores the addition of card values
            self.user.curr_points += self.user.first_card.value;
            self.user.curr_points += self.user.second_card.value;
            
            // Print playing cards
            self.print_playing_cards();

            if self.user.first_card.value == 1 && !(self.user.first_card.value == 1 && self.user.second_card.value == 10) {
                print!("Your first card is an Ace, would you want to count it as a 1 or 11? ");
                io::stdout().flush().expect("Failed to flush stdout");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                

                match input.as_str(){
                    "1" => self.user.first_card.value = 1,
                    "11" => self.user.first_card.value = 11,
                    _ => ()
                }
                self.print_playing_cards();
            } else if self.user.second_card.value == 1 && self.user.curr_points != 21 {
                println!("Your second card is an Ace, would you want to count it as a 1 or 11?");
                io::stdout().flush().expect("Failed to flush stdout");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                

                match input.as_str(){
                    "1" => self.user.second_card.value = 1,
                    "11" => self.user.second_card.value = 11,
                    _ => ()
                }
                self.print_playing_cards();
            } 
            
            if self.user.curr_points == 21 {
                self.control.game_state = GameState::Blackjack;
            } else if self.user.first_card.value == 1 || self.user.second_card.value == 1 {
                self.control.game_state = GameState::Ace;
            } else{
                self.control.game_state = GameState::Choosing;
            }

            match self.control.game_state {
                GameState::Blackjack => {

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

    fn print_playing_cards(&mut self) {
        println!("\n------ HOUSE ------");
        print!("First card: \n");
        self.house.first_card.print_card();
        print!("Second card: \n");
        self.house.second_card.print_blank_card();

        println!("\n------ USER ------");
        print!("First card: \n");
        self.user.first_card.print_card();
        print!("Second card: \n");
        self.user.second_card.print_card();
    }
}