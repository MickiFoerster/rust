struct MasterCard {
    number: u8,
    verification: u8,
}
struct Visa {
    number: u32
}
struct WesternUnion {
    name: String,
    verification: u8
}

impl CreditCharge for WesternUnion {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 3 == (self.verification % 2).into()
    }
}

struct BitCredit {
    btcnumber: u32
}

impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btcnumber %2
    }
}

trait CreditCharge {
    fn charge_with_id(&self, id: u32) -> bool ;
}

fn transact<Q: CreditCharge>(card: Q) {
    let id = 4096;
    if card.charge_with_id(id) {
        println!("Success!");
    } else {
        panic!("Invalid code!");
    }
}

fn main() {
    let card = BitCredit { btcnumber: 1024};
    /*
    let code = 4095;
    if card.charge_with_id(code) {
        println!("Success!");
    } else {
        println!("Failure!");
    }
    */
    transact(card);
}
