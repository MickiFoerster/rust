use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm,
    Nonce, // Or `Aes128Gcm`
};

fn main() {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let original_msg = b"{ Top secret message }";
    let ciphertext = cipher.encrypt(nonce, original_msg.as_ref()).unwrap();
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    println!(
        "{}, {}",
        std::str::from_utf8(&plaintext).unwrap(),
        std::str::from_utf8(original_msg).unwrap()
    );
    assert_eq!(&plaintext, original_msg);
}
