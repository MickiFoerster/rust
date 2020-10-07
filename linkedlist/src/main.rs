use std::collections::LinkedList;
use hello::say_hello;

mod hello {
    pub fn say_hello() {
        println!("Hello, World!")
    }
}

fn main() {
    println!("Hello linked list!");

    let mut ll = LinkedList::new();

    ll.push_back(2);
    ll.push_back(3);
    ll.push_back(5);

    println!("{:?}", ll);
    for elem in ll {
        println!("{}", elem);
    }

    println!("Now using vector:");
    
    let mut v = Vec::new();

    v.push("This");
    v.push("is");
    v.push("a");
    v.push("vector");

    println!("{:?}", v);

    say_hello()
}
