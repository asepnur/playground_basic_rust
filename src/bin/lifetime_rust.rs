
#[derive(Debug)]
struct obj{
    name:String,
    weight: i32,
}
fn main(){
    let a = "Testing";
    let b = "test";
    let ee:& str;
    {
        let c = longest(&a, &b);
        ee = &c;
    }
    println!("longest text is {} ", ee);
}
fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }else{
        y
    }
}