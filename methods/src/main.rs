
struct Person {
    forename: String,
    backname: String,
    age: u8,
}

impl Person {
    fn greeting(&self) -> String {
        format!("Hello {} {}, you are {} years old, right?", self.forename, self.backname, self.age)
    }
}

fn main() {
    let p = Person {
        forename: String::from("John"), 
        backname: String::from("Doo"), 
        age: 33
    };
    println!("{}", p.greeting());
}
