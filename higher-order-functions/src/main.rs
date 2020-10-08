fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn greater_than(limit: u32) 
    -> impl Fn(u32) -> bool {
    move |y| y > limit
}


fn main() {
    let limit = 500;
    let mut sum = 0;

    let above_limit = greater_than(limit);
    for i in 0.. {
        let isq = i*i;

        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("{}", sum);

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x:&u32| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("{}", sum2);
}
