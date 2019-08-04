fn main(){
    let p = reverse((32, true));
    println!("value of p1: {}\n p2: {}\n", p.0, p.1);
    let t1 = T(1, true);
    let t2 = t1.reverse();
    println!("value of t2: {:?}", t2);

    let tt1 = T(1, true);
    let tt2 = tt1;
    println!("tt1: {:?}\ntt2:{:?}", tt1, tt2);
}
#[derive(Debug, Copy, Clone)]
struct T (i32, bool);

// uncomment the code bellow will produce an error, due to Copy could not implement to String
// #[derive(Debug, Copy, Clone)]
struct S (String, bool);

fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (interger, boolean) = pair;
    (boolean, interger)
}
impl T {
    fn reverse(&self)-> (bool, i32){
        (self.1, self.0)
    }
}