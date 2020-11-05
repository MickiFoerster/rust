use std::rc::Rc;

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new("John".to_string());
    println!("Name = {}, reference count: {}", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone()); // clone() increases the reference count
        person.greet();
        println!("Name = {}, reference count: {}", name, Rc::strong_count(&name));
    }
    println!("Name = {}, reference count: {}", name, Rc::strong_count(&name));

    println!("Name = {}", name);
}

fn main() {
    rc_demo()
}
