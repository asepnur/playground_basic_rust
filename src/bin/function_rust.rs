extern crate playground_basic_rust;
use playground_basic_rust::circle;
fn main(){
    let c = circle::CircleBuilder::new()
        .x(12.0)
        .y(12.0)
        .radius(12.0)
        .finalize();
    println!("circle c x: {}", c.x);
    println!("circle c y: {}", c.y);
    println!("circle c radius: {}", c.radius);
    println!("circle c area: {}", c.area());
}