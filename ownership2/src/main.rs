fn main() {
    let v = vec![1,2,3];
    //let v2 = v;

    let foo = |v:Vec<i32>| ();
    foo(v);
    //println!("{:?}", v); cannot use v anymore since it was moved before
    //
    let u = 1;
    let u2 = u;
    println!("{}", u); // This works since u is a base type not a type where a pointer is involved

    let u = Box::new(1);
    let u2 = u;
    //println!("{}", u);  This is the same as above. u was moved to u2.
    
    // Solution where we take over ownership and return ownership back
    let print_vector = |x:Vec<i32>| -> Vec<i32>
    {
        println!("{:?}", x);
        x
    };

    let v2 = vec![1,2,3];
    let v2 = print_vector(v2);
    println!("{:?}", v2);

}
