struct Person {
    name : String
}

impl Person 
{
    //fn new<S: Into<String>>(name: S) -> Person 
    fn new<S>(name: S) -> Person  where S: Into<String>  // alternative definition
    {
        Person {name: name.into()}
    }
}

fn main() {
    let john = Person::new("John");

    let name: String = "Jane".to_string();
    let jane = Person::new(name/*.as_ref()*/);
}
