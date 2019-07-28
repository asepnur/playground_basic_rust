pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(val) => Some(val + 1),
        None => None,
    }
}

// _ placeholder
pub fn match_with_placeholder(i: u8) {
    match i {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("over the limit"),
    }
}
