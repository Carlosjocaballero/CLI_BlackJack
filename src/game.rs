use std::io::{self, Write, Read};
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

            self.new_round();
            

            // Check if an ace is part of the user's cards
            self.evaluate_ace();
            

            if self.check_user_blackjack() {
                self.control.game_state = GameState::Blackjack;
            } else{
                self.control.game_state = GameState::Choosing;
            }

            match self.control.game_state {
                GameState::Blackjack => {
                    self.print_playng_cards();
                    self.print_curr_score();
                    println!("BlackJack!");
                    self.control.game_state = GameState::Win;
                },
                GameState::Choosing => {
                    self.choosing();
                },
                _ => ()
            }

            match self.control.game_state {
                GameState::Win => {
                    println!("You won!");
                    self.user.total_money += self.user.curr_bet;
                },
                GameState::Lose => {
                    println!("You lost!");
                    self.user.total_money -= self.user.curr_bet;
                },
                _ => ()
            }

            self.summary();
            

        }

        
    }

    fn new_round(&mut self){

        // Prompt user for bet amount
        print!("\nHow much do you want to bet? ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Read input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Set as bet amount
        self.user.curr_bet = input.trim().parse().unwrap();

        // Reset points for house and user
        self.house.curr_points = 0;
        self.user.curr_points = 0;

        while self.house.cards.len() != 2 {
            self.house.cards.pop();
        }

        // House draws 2 cards
        self.house.cards[0] = self.draw_card();
        self.house.cards[1] = self.draw_card();

        // House stores the addition of card values
        self.house.curr_points += self.house.cards[0].value;
        self.house.curr_points += self.house.cards[0].value;

        while self.user.cards.len() != 2 {
            self.user.cards.pop();
        }

        // User draws 2 cards
        self.user.cards[0] = self.draw_card();
        self.user.cards[1] = self.draw_card();

        // ---- TESTING ACE ----
        // self.user.cards[0] = Card::new('♥', 9);
        // self.user.cards[1] = Card::new('A', 1);

        // User stores the addition of card values
        self.user.curr_points += self.user.cards[0].value;
        self.user.curr_points += self.user.cards[1].value;
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

    fn print_playng_cards(&mut self){
        println!("\n------ HOUSE ------");
        for i in 0..self.house.cards.len(){
            print!("Card #{}: \n", i+1);
            self.house.cards[i].print_card();
        }

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
                self.user.curr_points += 10;
            }
        }
    }

    fn summary(&mut self){
        println!("You have {} credits.", self.user.total_money);
    }

    fn choosing(&mut self){
        while self.control.game_state == GameState::Choosing {
            // Print playing cards
            self.print_playing_cards_hidden();

            print!("You have {} points. What would you want to do? 1.Hit  2.Stand  3.Quit Game ", self.user.curr_points);
            io::stdout().flush().expect("Failed to flush stdout");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read user choice");

            let mut user_choice: i8 = input.trim().parse().unwrap();

            match user_choice {
                1 => {
                        let new_card: Card = self.draw_card();
                        self.user.curr_points += new_card.value;
                        self.user.cards.push(new_card);
                    
                        if self.user.curr_points > 21 && self.user.has_ace{
                            self.user.cards[self.user.ace_pos].value = 1;
                            self.user.curr_points -= 10;
                            self.user.has_ace = false;
                        }

                        if self.user.curr_points > 21 {
                            self.print_playng_cards();
                            self.print_curr_score();
                            println!("You busted!");
                            self.control.game_state = GameState::Lose;
                        }
                },
                2 => {
                    self.determine_game();
                },
                3 => {
                    self.control.game_state = GameState::End;
                },
                _ => println!("{} is not a valid answer. Please try again.\n", user_choice)
            }
        }
    }

    fn determine_game(&mut self){        
        while self.house.curr_points < 18 && self.house.curr_points != 21{
            let new_card: Card = self.draw_card();
            self.house.curr_points += new_card.value;
            self.house.cards.push(new_card);
        } 
        self.print_playng_cards();
        self.print_curr_score();
        // Assume that user score is < 21 and won't increase anymore
        if self.user.curr_points > 21{
            println!("You busted!");
            self.control.game_state = GameState::Lose;
        }
        else if self.house.curr_points > 21{
            println!("House busted!");
            self.control.game_state = GameState::Win;
        }
        else if self.user.curr_points > self.house.curr_points{
            self.control.game_state = GameState::Win;
        } else{
            self.control.game_state = GameState::Lose;
        }
    }

    fn print_curr_score(&mut self){
        println!("House: {}  You: {}", self.house.curr_points, self.user.curr_points);
    }
}