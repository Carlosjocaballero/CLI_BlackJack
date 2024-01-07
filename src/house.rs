use crate::card::Card;

pub struct House{
    pub curr_points: i16,
    pub first_card: Card,
    pub second_card: Card
}

impl House{
    pub fn new() -> Self{
        House{
            curr_points: 0,
            first_card: Card::new('A', 1),
            second_card: Card::new('A', 1)
        }
    }
}