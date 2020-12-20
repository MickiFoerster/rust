use ssh2::Session;

fn main() {
    // Almost all APIs require a `Session` to be available
    let sess = Session::new().unwrap();
    let mut agent = sess.agent().unwrap();

    // Connect the agent and request a list of identities
    agent.connect().unwrap();
    agent.list_identities().unwrap();

    for identity in agent.identities().unwrap() {
        println!("{}", identity.comment());
        let _pubkey = identity.blob();
    }
}

