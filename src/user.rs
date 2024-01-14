use crate::card::Card;

pub struct User{
    pub curr_points: i16,
    pub total_points: i16,
    pub cards: Vec<Card>,
    pub has_ace: bool,
    pub ace_pos: usize
}

impl User{
    pub fn new() -> Self{
        let mut cards: Vec<Card> = Vec::new();
        cards.push(Card::new(' ',0));
        cards.push(Card::new(' ',0));
        User{
            curr_points: 0,
            total_points: 0,
            cards: cards,
            has_ace: false,
            ace_pos: 0
        }
    }
}