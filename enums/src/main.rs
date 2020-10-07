#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat
}

enum Relationship {
    Father,
    Mother,
    Daugther,
    Son,
    Sibling,
    Other()
}

fn main() {
    let my_pet = Animal::Dog;
    let other_pet = Animal::Cat;

    assert!(my_pet == other_pet);
}
