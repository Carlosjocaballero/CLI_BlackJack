use std::vec;

use crate::card::Card;

pub struct Cards{
    pub deck: Vec<Card>
}

impl Cards{


    pub fn new() -> Self{
        let mut deck : Vec<Card> = Vec::new(); 
        // '♥', '♦', '♣', '♠'
        let syms: Vec<char> = vec!['♥', '♦', '♣', '♠'];
        let other_syms: Vec<char> = vec!['K', 'Q', 'J'];
        
        for _sym in &syms{
            for _i in 2..11{
                deck.push(Card::new(*_sym, _i));
            }

            for oth_sym in &other_syms{
                deck.push(Card::new(*oth_sym, 10));
            }

            deck.push(Card::new('A', 1));
        }

        Cards { deck: deck }
    }

    pub fn print_deck(&mut self){
        for card in &mut self.deck {
            card.print_card();
        }
    }
    // pub fn add()
}

