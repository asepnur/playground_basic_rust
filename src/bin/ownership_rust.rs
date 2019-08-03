// rules of ownership
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


// to enable copy, added derive Copy and Clone to your struct
#[derive(Debug, Copy, Clone)]
struct Foo {
    age: i32,
}
fn main(){

    let x = 1;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let z = String::from("this is z");
    let w = z;
    // z is not hold value anomore, because
    // the ownership of z has assigned to w
    // this code will produce error
    // println!("w: {}, z: {}", w, z);
    println!("w: {}", w);

    // there is a way to do that
    // using shallow copy with .clone trait
    // this will move not just stack data
    // using clone manually 
    let xx = String::from("this is xx");
    let mut yy = xx.clone();
    yy = String::from("this is now yy");
    println!("xx: {}, yy: {}", xx, yy);
    
    // using Copy trait
    let f:Foo = Foo{
        age: 12,
    };
    let o:Foo = f;
    println!("f: {:?}, o:{:?}", f, o);
}