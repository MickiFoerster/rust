use redis::Commands;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = con
        .set(
            "my_key",
            "asdjfljsdlkfjasldfkgjalksjflkajsdlkfjaskldjfklasjdflksdj",
        )
        .unwrap();
    let value: String = con.get("my_key").unwrap();
    println!("{}", value);
}
