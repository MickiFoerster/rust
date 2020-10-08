fn say_hello() { println!("hello"); }

fn closures() {
    let f = say_hello;
    f();

    let plus_one = |x:i32| -> i32 { x + 1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }

    let borrow_two = &mut two;
    println!("{}", borrow_two);

    let plus_three = |x:&mut i32| *x += 3;
    let mut x = 100;
    plus_three(&mut x);
    println!("{}", x);
}

fn main() {
    closures();
}
