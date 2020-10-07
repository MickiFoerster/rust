//fn take_ownership_sum(v: Vec<i32>) -> i32 {
//    let mut sum = 0;
//    for value in v {
//        sum += value;
//    }
//    return sum;
//}
//
//fn borrow_sum(v: &Vec<i32>) -> i32 {
//    let mut sum = 0;
//    for value in v {
//        sum += value;
//    }
//    return sum;
//}
//
//fn main() {
//    let values = vec![1,2,3,4,5];
//    let sum = borrow_sum(&values);
//    println!("Sum of {} values: {}", values.len(), sum);
//}

//fn cap_values_owned(max: i32, mut v: Vec<i32>) -> Vec<i32> {
//    for index in 0..v.len() {
//        if v[index] > max {
//            v[index] = max;
//        }
//    }
//    return v;
//}

//fn cap_values_borrow(max: i32, v: &mut Vec<i32>) {
//    for index in 0..v.len() {
//        if v[index] > max {
//            v[index] = max;
//        }
//    }
//}
//
//fn main() {
//    let mut values = vec![1,2,3, 10000, 5];
//    cap_values_borrow(10, &mut values);
//
//    for v in values {
//        println!("{}", v);
//    }
//}

fn main() {
    let mut values = vec![1, 2, 3, 4, 5];

    let a = &values; 
    let b = &values;

    values[2] = 2000000;
}
