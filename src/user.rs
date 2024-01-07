use crate::card::Card;

pub struct User{
    pub curr_points: i16,
    pub total_points: i16,
    pub first_card: Card,
    pub second_card: Card
}

impl User{
    pub fn new() -> Self{
        User{
            curr_points: 0,
            total_points: 0,
            first_card: Card::new(' ',0),
            second_card: Card::new(' ', 0)
        }
    }
}