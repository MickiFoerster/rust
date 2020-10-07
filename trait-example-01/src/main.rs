struct MyType {
    x: u32
}

trait Print {
    fn print(&self);
}

impl Print for MyType {
    fn print(&self) {
        println!("{}", self.x);
    }
}

fn main() {
    let a = MyType { x:23 };
    a.print();
}
