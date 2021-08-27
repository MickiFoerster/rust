use serde_derive::*;


pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Transaction {
    from: String,
    to : String,
    amount: u64,
}

fn main() {
    println!("Hello, world!");
    let trans = get_transactions("test_data/transactions.json").expect("Could not load transactions");

    for t in trans {
        println!("{:?}", t);
    }
}

fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.into())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))
}
