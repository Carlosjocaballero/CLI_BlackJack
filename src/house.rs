use crate::card::Card;

pub struct House{
    pub curr_points: i16,
    pub cards : Vec<Card>,
}

impl House{
    pub fn new() -> Self{
        let mut cards: Vec<Card> = Vec::new();
        cards.push(Card::new(' ', 0));
        cards.push(Card::new(' ', 0));

        House{
            curr_points: 0,
            cards: cards
        }
    }
}