extern crate playground_basic_rust;
// dont use this in production
use self::playground_basic_rust::*;
use self::rectangle::Rectangle;
fn main(){
    let width:i32 = 10;
    let height:i32 = 10;
    // call defining method
    let mut rec = Rectangle::new(width, height);
    let area = rec.area();
    rec.print_area(area);
    rec.set_width(12);
    rec.set_height(12);
    rec.print_area(rec.area());

    // Associated functions
    let rec_a = Rectangle::new_squere(width - 5);

    rec_a.print_area(rec_a.area());
    // Method with more paramter
    println!("is rec can hold rec_a ? {}",rec.can_hold(&rec_a));
}