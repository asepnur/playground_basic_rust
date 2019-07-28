extern crate playground_basic_rust;
use self::playground_basic_rust::*;

fn main(){
    // ip address, simple enum
    let home = ip_address::IpAddrKind::V4(0, 0, 0, 1);
    let loop_back = ip_address::IpAddrKind::V6(String::from(":1"));
    ip_address::kind_of_ip(home);
    ip_address::kind_of_ip(loop_back);

    // coin
    let p = coin::Coin::Penny;
    let q = coin::Coin::Quarter(coin::UsState::Alabama);
    println!("p has value in cent {}", p.value_in_cent());
    println!("q has value in cent {}", q.value_in_cent());

    // enum with option value
    let five = Some(5);
    println!("five is {:?}", five);
    let six = options_value::plus_one(five);
    println!("six is {:?}", six);
    let none = options_value::plus_one(None);
    println!("none is {:?}", none);
    // using _ placeholder
    let one:u8 = 1;
    let ten:u8 = 10;
    options_value::match_with_placeholder(one);
    options_value::match_with_placeholder(ten);

    // pattern matching using if let
    let mut count = 0;
    if let coin::Coin::Quarter(state) = p {
        println!("State quater from {:?}", state);
    }else{
        count += 1;
    }
    println!("count: {}",count);
}
