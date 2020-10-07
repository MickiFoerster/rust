use std::collections::HashSet;

fn main() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("gamma");
    println!("{:?}", greeks);

    let ok = greeks.insert("vega");
    if ok {
        println!("We added value to set!");
    }
    let ok = greeks.insert("gamma");
    if ok {
        println!("We added value to set!");
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let ok = greeks.remove("delta");
    if ok {
        println!("We removed delta!");
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("is {:?} a subset of {:?}? {:?}", _2_8, _1_10, _2_8.is_subset(&_1_10));
    println!("is {:?} disjoint to {:?}? {:?}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));
    println!("{:?} join with {:?} is {:?}", _2_8, _6_10, _2_8.union(&_6_10));
}
