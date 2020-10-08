fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 { x*y }

fn functions() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);
    print_value(z);

    print_value(product(2,3));
}

fn main() {
    functions();
}
