use mock_example::{Dummy, SecretSequence};

fn main() {
    let dummy = Dummy;
    for _ in 1..=10 {
        println!("{}", dummy.next_number());
    }
    println!("Now the trait:");
    let sec = &dummy as &dyn SecretSequence;
    for _ in 1..=10 {
        println!("{}", sec.next_number());
    }
}
