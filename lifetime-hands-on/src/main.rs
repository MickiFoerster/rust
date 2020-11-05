struct Person<'a> 
{
    name: &'a str
}

impl<'b> Person<'b>  // This means that code needs to live as long as the 
                     // structure Person lives.
{
    fn talk(&self) 
    {
        println!("Hi, my name is {}.", self.name);
    }
}

fn main() {
    let person = Person { name: "micki" };
    person.talk();
}
