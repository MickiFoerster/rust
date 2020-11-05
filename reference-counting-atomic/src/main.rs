use std::thread;
use std::sync::Arc;

struct Person {
    name: Arc<String>
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn rc_demo() {
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone()); // clone() increases the reference count
    let t = thread::spawn(move || {
        person.greet(); // thread print greeting
    });
    println!("Name = {}", name); // main thread should print name

    t.join().unwrap(); // join thread
}

fn main() {
    rc_demo()
}
