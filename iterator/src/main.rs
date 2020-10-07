fn main() {
    let vec = vec![3, 2, 1];
    for x in vec.iter() {
        println!("{}", x);
    }

    // mutable iterator
    let mut vec_mutable = vec![3, 2, 1];
    for x in vec_mutable.iter_mut() {
        *x *= 2;
        println!("{}", x);
    }

    // reverse order
    for x in vec.iter().rev() {
        println!("{}", x);
    }

    // into_iter
    let it = vec.into_iter();
    //println!("{:?}", vec); // error: value borrowed here after move
    println!("{:?}", it);
    for x in it {
        println!("{}", x);
    }
}
