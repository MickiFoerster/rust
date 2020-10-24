use std::fmt::Debug;

#[derive(Debug)]
struct Square {
    side: f64
}

#[derive(Debug)]
struct Circle {
    radius: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

//fn print_info(shape: impl Shape + Debug) {
//fn print_info<T: Shape + Debug>(shape: T) { // useful if multiple arguments
fn print_info<T>(shape: T) where T: Shape + Debug
{ // useful if multiple arguments
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn main() {
    let s = Square{side: 2.};
    let c = Circle{radius: 3.141529};
    println!("square: {}", s.area());
    println!("circle: {}", c.area());
    print_info(s);
    print_info(c);
}
