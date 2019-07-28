extern crate playground_basic_rust;
use self::playground_basic_rust::*;

fn main(){
    let width:i32 = 10;
    let height:i32 = 10;
    let mut rec = rectangle::new(width, height);
    let area = rec.area();
    rec.print_area(area);
    rec.set_width(12);
    rec.set_height(12);
    rec.print_area(rec.area());

    let rec_a = rectangle::new(width - 5, height- 5);
    rec_a.print_area(rec_a.area());
    println!("is rec can hold rec_a ? {}",rec.can_hold(&rec_a));
}