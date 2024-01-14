// Create a struct for controls. Use this to keep track of current state, screen type (menu, game, credits, etc)
#[derive(PartialEq)]
pub enum GameState{
    Dealing,
    Choosing,
    Blackjack,
    Win,
    Lose,
    End
}

pub struct Control{
    pub game_state: GameState
}

impl Control{
    pub fn new() -> Self{
        Control{
            game_state: GameState::Dealing
        }
    }
}