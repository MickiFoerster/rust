use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::new();

    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides",
             shapes["square"]);

    println!("{:?}", shapes);

    shapes.insert("square".into(), 111);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    // insert if there is not already a value
    shapes.entry("circle".into()).or_insert(1);

    {
        let actual = shapes.entry("circle".into()).or_insert(11111);
        *actual = 0;
    }
}
