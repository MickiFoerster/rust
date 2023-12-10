use serde_derive::Serialize;
use std::io::prelude::*;

#[derive(Serialize)]
struct Address {
    forename: String,
    backname: String,
    street: String,
    city: String,
}

fn main() {
    let address = Address {
        forename: String::from("John"),
        backname: String::from("Dooh"),
        street: String::from("Mainstreet"),
        city: String::from("Springfield"),
    };

    let as_bincode = bincode::serialize(&address).unwrap();
    let mut f = std::fs::File::create("address.bin").expect("Could not create file");
    f.write_all(&as_bincode).expect("Could not write to file");
    drop(f);

    let mut f = std::fs::File::open("address.bin").expect("Could not open file");
    let metadata = std::fs::metadata("address.bin").expect("Unable to read metadata");
    let mut buffer: [u8; _] = [0u8; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    let address2 = bincode::deserialize(buffer);
}
