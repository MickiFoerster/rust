fn main() {
    let mut v = Vec::new();

    v.push(2);
    v.push(3);
    v.push(5);

    println!("v = {:?}", v);

    v.push(7);
    println!("v = {:?}", v);
    println!("v[0] = {}", v[0]);

    for idx in 0..10 {
        match v.get(idx) {
            Some(x) => println!("{}", x),
            None => println!("illegal index"),
        }
    }

    v.push(23);
    println!("{:?}", v);
    let last_elem = v.pop(); // Option
    println!("last elem is {:?}, v = {:?}", last_elem, v);

    // As long as we get Some(x) and not None print elements
    while let Some(x) = v.pop() {
        println!("{}", x)
    }

}
