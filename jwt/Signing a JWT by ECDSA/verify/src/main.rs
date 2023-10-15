use josekit::{
    jws::ES256,
    jwt::{self},
    JoseError,
};

const PUBLIC_KEY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../ES256-public.pem");

fn main() -> Result<(), JoseError> {
    let jwt = std::fs::read("../jwt").unwrap();

    // Verifing JWT
    let public_key = std::fs::read(PUBLIC_KEY).unwrap();
    let verifier = ES256.verifier_from_pem(&public_key)?;
    let (payload, header) = jwt::decode_with_verifier(&jwt, &verifier)?;

    println!("Signature is valid!");
    println!("header: {header}");
    println!("payload: {payload}");

    Ok(())
}
