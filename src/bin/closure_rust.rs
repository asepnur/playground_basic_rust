fn main(){
    // closure and specify parameter
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    let two = plus_one(1);
    println!("two: {}", two);

    // closure with doesn't specify paramter type
    let plus_two = |x| {
        let mut result: i32 = x; 
        result += 1;
        result += 1;
        result
    };
    let six = plus_two(4);
    println!("six: {}", six);

    // closure and paramter local binding
    let num = 5;
    let plus_num = |x| x +num;
    let seven = plus_num(2);
    println!("seven: {}", seven);
    println!("num: {}", num);

    // move closure
    let mut num_1 = 5;
    {
        let mut owns_num_1 = move |x: i32|{
            num_1 = 10;
            println!("num_1: {}", num_1);
            x + num_1
        }; 
        let eight = owns_num_1(3);
        println!("eight: {}",eight);
        println!("num_1: {}", num_1);
    }
    println!("num_1: {}", num_1);

    // same scenario without move closure
    let mut num_2 = 5;
    {
        let mut owns_num_2 = |x: i32| {
            num_2 = 10;
            println!("num_2: {}", num_2);
            x + num_2
        };
        let ten = owns_num_2(5);
        println!("ten: {}", ten);
        println!("num_2: {}", num_2);
    }
    println!("num_2: {}", num_2);

    // taking closure as argument
    let answer = call_with_one(&|x| x + 2);
    println!("answer: {}", answer);

    let answer_2 = call_with_ref(&|x| x + 10);
    println!("answer_2: {}", answer_2);


    let f = factory();
    println!("factory: {}", f(1));
}

fn call_with_one<F>(some_closure: &F) -> i32 
    where F: Fn(i32) -> i32{

    some_closure(1)
}
fn call_with_ref<F>(some_closure:&F) -> i32
    where F: for<'a> Fn(&'a i32) -> i32 {
    let value = 0;
    some_closure(&value)
}

fn factory() -> Box<Fn(i32) -> i32>{
    let num = 5;
    Box::new(move |x| x + num)
}