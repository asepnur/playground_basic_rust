
#[derive(Debug)]
struct foo<'a>{
    x:&'a i32,
}
fn main(){
    let ee:& str;
    {
            let a = &"Testing";
            let b = &"test";
            let c = compare(&a, &b);
            ee = &c;
    }
    println!("longest text is {} ", ee);

    let x;
    {
        let y = &5;
        let f = foo{x: y};
        x = &f.x;
        println!("{}", x);
    }
    // println!("{}", x);
}
fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
fn compare<'a>(x:&'a str, y:&'a str) -> &'a str {
    longest(x, y)
}