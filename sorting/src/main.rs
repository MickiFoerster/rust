fn main() {
    let v: Vec<u8> = vec![2, 5, 1, 6, 8, 4, 4, 10, 234, 123, 88, 33, 54, 42];
    let mut v1: Vec<u8> = vec![];
    for i in 0..v.len() {
        v1.push(v[i]);
    }
    println!("{:?}", v1);
    insertion_sort(&mut v1);
    println!("{:?}", v1);

    let mut v2: Vec<u8> = vec![];
    for i in 0..v.len() {
        v2.push(v[i]);
    }
    println!("{:?}", v2);
    insertion_search_sort_with_binary_search(&mut v2);
    println!("{:?}", v2);
}

fn insertion_sort(v: &mut Vec<u8>) {
    for i in 1..v.len() {
        let t = v[i];
        let mut j = i;
        while j > 0 && v[j - 1] > t {
            v[j] = v[j - 1];
            j = j - 1;
        }
        v[j] = t
    }
}

fn insertion_search_sort_with_binary_search(v: &mut Vec<u8>) {
    for i in 1..v.len() {
        let t = v[i];
        // binary search for insertion location since 0..i-1 is already sorted
        let mut l = 0;
        let mut r = i;
        while l < r {
            let m = (l + r) / 2;
            if v[m] < t {
                l = m + 1;
            } else {
                r = m;
            }
        }
        // Now r points to the location where t is to be inserted
        // Move all elements r..i one position to the right
        let mut j = i;
        while j > r {
            v[j] = v[j - 1];
            j = j - 1;
        }
        v[r] = t
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_insertion_sort() {
        println!("HELLO");
        let mut f = File::open("/dev/urandom").expect("Unable to open /dev/urandom");
        let mut pos = 0;
        let mut buffer = [0; 4096];
        match f.read(&mut buffer) {
            Err(e) => panic!("{:?}", e),
            Ok(n) => println!("{} bytes read", n),
        }
        let mut v: Vec<u8> = vec![];
        for i in 0..buffer.len() {
            v.push(buffer[i]);
        }
        super::insertion_sort(&mut v);
        println!("{:?}", v);
    }
}
