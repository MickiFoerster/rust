fn main() {
    let print_vector = |x:&Vec<i32>|
    {
        println!("x[0] = {}", x[0]);
    };

    let v = vec![3,2,1];
    print_vector(&v);

    // After vector have been borrowed you can use is further
    print_vector(&v);


    let mut a = 40;
    let b = &mut a;
    *b += 2;
    println!("a = {}", a); // imutable borrow here
    //println!("b = {}", b); // mutable borrow here -> FAILS
    
    
    let mut z = vec![3,2,1];

    for i in &z { // immutable borrow occurs here
        println!("i = {}", i);
        //z.push(5); mutable borrow occurs here -> FAILS
    }
}
