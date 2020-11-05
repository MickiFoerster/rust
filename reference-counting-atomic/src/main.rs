use std::thread;
use std::sync::{Arc,Mutex};

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>> // If you need to change the component then use Mutex
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name: name, state: state }
    }

    fn greet(&self) {

        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I'm {}", self.name, state.as_str());
    }
}

fn rc_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone()); // clone() increases the reference count
    let t = thread::spawn(move || {
        person.greet(); // thread print greeting
    });
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str()); // main thread should print name

    t.join().unwrap(); // join thread
}

fn main() {
    rc_demo()
}
