use std::vec;

use crate::card::Card;

pub struct Cards{
    pub stack: Vec<Card>
}

impl Cards{


    pub fn new() -> Self{
        let mut stack : Vec<Card> = Vec::new(); 
        // '♥', '♦', '♣', '♠'
        let syms: Vec<char> = vec!['♥', '♦', '♣', '♠'];
        let other_syms: Vec<char> = vec!['K', 'Q', 'J'];
        
        for _sym in &syms{
            for _i in 2..11{
                stack.push(Card::new(*_sym, _i));
            }

            for oth_sym in &other_syms{
                stack.push(Card::new(*oth_sym, 10));
            }

            stack.push(Card::new('A', 1));
        }

        Cards { stack: stack }
    }

    pub fn print_deck(&mut self){
        for card in &mut self.stack {
            card.print_card();
        }
    }
    // pub fn add()
}

