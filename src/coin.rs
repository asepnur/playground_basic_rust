#[derive(Debug)]
pub enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
pub enum UsState{
    Alabama,
    Alaska,
}

impl Coin{
    pub fn value_in_cent(&self) -> u8 {
        match self{
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quater from {:?}!", state);
                25
            }
        }
    }
}