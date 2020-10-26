trait Animal
{
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}
struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

fn main() {
    let mut creatures = Vec::new();
    //creatures.push(Human{name: "John"});
    //creatures.push(Cat{name: "Fluffy"}); // error due to expected type Human

    creatures.push(Creature::Human(
            Human {name: "John"}
    ));
    creatures.push(Creature::Cat(
            Cat {name: "Fluffy"}
    ));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }


    // Alternative to the enum approach:
    let mut animals:Vec<Box<Animal>> = Vec::new();
    animals.push(
        Box::new(Human{name: "John"})
    );
    animals.push(
        Box::new(Cat{name: "Fluffy"})
    ); 

    for a in animals.iter() {
        a.talk();
    }
}
