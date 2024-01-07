
#[derive(Clone)]
pub struct Card {
    value : i8,
    symbol : char
} 

impl Card{
    pub fn new(symbol:char, value:i8)-> Self{
        Card {
            value: value,
            symbol: symbol
        }
    }
    pub fn print_card(&mut self){
        for _i in 0..6{
            print!("x");
        }
        println!("");
        print!("x {}  x\n", self.symbol);
        if self.value == 10{
            print!("x {} x\n", self.value);
        } else {
            print!("x {}  x\n", self.value);
        }
        for _i in 0..6{
            print!("x");
        }
        println!("");
        // "x x x"
        // "x x x"
        // "xxxxx"
    }

}