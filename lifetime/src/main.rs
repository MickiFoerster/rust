#[derive(Debug)]
struct Person 
{
    name: String
}

#[derive(Debug)]
struct Company<'z>
{
    name: String,
    ceo: &'z Person // Reference to Person lives as long as Company lives
}

impl Person
{
    fn get_ref_name(&self) -> &String
    // What you actually get from the above signature is
    // fn get_ref_name<'a>(&'a self) -> &'a String
    // This means all have the same lifetime.
    {
        &self.name
    }
}

fn main() {
    // The 'static' in &'static is a lifetime. In the following the string
    // lives as long as the program lives. 'static' is a special lifetime.
    // You can also use your own lifetimes.
    let _s: &'static str = "Test";


    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };
    println!("{:?}", tesla);


    let z: &String;
    {
        let p = Person { name: String::from("John") };
        z = p.get_ref_name();
        println!("z = {}", z);
    }
    //println!("z = {}", z);
}
