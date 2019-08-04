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

    // owneship in function scope
    let s = String::from("its me depelover");
    take_ownership(s);
    // code bellow will produce error if you try to uncomment
    // println!("s value is {}", s);

    let i:i32 = 100;
    makes_copy(i);
    // the code bellow not produce error because i is i32 (fixed/known size) it should be go to stack
    println!("i value is {}", i);

    let ss = give_ownership();
    println!("ss value is {}", ss);
    
    let xx = takes_and_gives_back(ss);
    println!("xx value is {}", xx);
    // trying to use ss anymore will produce an error
    // because ownership of s's has given to xx
}

// ownership using function
fn take_ownership(s: String){
    println!("value {} is taken", s);
}

fn makes_copy(some_integer: i32){
    println!("value {} is copied", some_integer);
}

fn give_ownership() -> String {
    String::from("value from give_ownership function")
}
fn takes_and_gives_back(s: String) -> String{
    s
}
