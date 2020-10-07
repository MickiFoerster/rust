struct Foo {
    quax: i32,
    baz: String,
    z: Bar,
}

struct Bar {
    num: i32,
}

struct Point2D {
    x: f64,
    y: f64,
}

fn add_points(a: Point2D, b: Point2D) -> Point2D {
    Point2D {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

fn main() {
    let a = Foo {
        quax: 10,
        baz: String::from("Hello World!"),
        z: Bar {
            num: 23,
        }
    };
    println!("{}", a.quax);
    println!("{}", a.baz);
    println!("{}", a.z.num);

    let p1 = Point2D { x: 10.0, y: 20.0 };
    let p2 = Point2D { x: -2.0, y: 30.5 };
    println!("{}", add_points(p1, p2).x);
}
