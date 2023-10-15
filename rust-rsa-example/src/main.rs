use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey,
};

fn main() {
    let mut rng = rand::thread_rng();

    let bits = 1024;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    println!("private key has been generated");
    let public_key = RsaPublicKey::from(&private_key);
    println!("public key has been derived frorm private key");
    let encoded_private_key_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .expect("encoding private key failed");
    let encoded_public_key_pem = public_key
        .to_public_key_pem(LineEnding::LF)
        .expect("encoding public key failed");

    std::fs::write("private-only-1024bits.pem", encoded_private_key_pem)
        .expect("write to file failed");
    std::fs::write("public.pem", encoded_public_key_pem).expect("write to file failed");

    let pem = std::fs::read("private-only-1024bits.pem").expect("load PEM from file failed");
    let pem = std::str::from_utf8(&pem).expect("invalid UTF-8");
    let private_key = RsaPrivateKey::from_pkcs8_pem(pem).expect("private key loading failed");

    let pem = std::fs::read("public.pem").expect("load PEM from file failed");
    let pem = std::str::from_utf8(&pem).expect("invalid UTF-8");
    let public_key = RsaPublicKey::from_public_key_pem(pem).expect("public key loading failed");

    // Encrypt
    let data = br#"{ "msg" : "This is top secret" }"#;
    let enc_data = public_key
        .encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = private_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);

    let dec_data_utf8 = std::str::from_utf8(&dec_data).expect("invalid UTF-8");
    println!("{}", dec_data_utf8);
}
