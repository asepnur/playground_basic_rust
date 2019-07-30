pub mod back_of_house{
    pub fn kitchen(){
        // your logic here
        println!("state of kitchen accessed");
    }
}

pub mod front_of_house {
    // bring all pub mod inside accessed to front_of_house
    pub use super::back_of_house;

    pub mod hosting{
        pub fn add_to_whitelist(){
            // your logic here
            println!("added to whitelist");
        }
    }
    pub mod desk{

    }
    pub mod waiter{
        pub fn say_welcome(){
            println!("welcome to our resto");
        }
    }
    pub fn access_back_of_house(){
        back_of_house::kitchen();
    }
}
pub fn eating_at_restaurant(){
    // bring hosting and waiter to this package
    use front_of_house::{hosting, waiter};
    self::back_of_house::kitchen();
    hosting::add_to_whitelist();
    waiter::say_welcome();
}
