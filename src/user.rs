
pub struct User{
    pub curr_points: i16,
    pub total_points: i16
}

impl User{
    pub fn new() -> Self{
        User{
            curr_points: 0,
            total_points: 0
        }
    }
}