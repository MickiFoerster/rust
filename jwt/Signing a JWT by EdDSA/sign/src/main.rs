use josekit::{
    jws::{EdDSA, JwsHeader},
    jwt::{self, JwtPayload},
    JoseError,
};

const PRIVATE_KEY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../ED25519-private.pem");

fn main() -> Result<(), JoseError> {
    let mut header = JwsHeader::new();
    header.set_token_type("JWT");

    let mut payload = JwtPayload::new();
    payload.set_subject("subject");

    // Signing JWT
    eprintln!("private key: {}", PRIVATE_KEY);
    let private_key = std::fs::read(PRIVATE_KEY).unwrap();
    let signer = EdDSA.signer_from_pem(&private_key)?;
    let jwt = jwt::encode_with_signer(&payload, &header, &signer)?;

    std::fs::write("../jwt", jwt).expect("write jwt failed");

    println!("Signed JWT written to file ../jwt");

    Ok(())
}
