use hex;

fn main() -> Result<(), aes_gcm::Error> {
    use aes_gcm::{
        Aes256Gcm,
        aead::{Aead, AeadCore, KeyInit, OsRng},
    };

    let key = Aes256Gcm::generate_key(OsRng);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, b"hellow there !".as_ref())?;

    println!("ciphertext (hex): {}", hex::encode(&ciphertext));

    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;

    println!("plaintext: {}", std::str::from_utf8(&plaintext).unwrap());

    assert_eq!(&plaintext, b"hellow there !");

    Ok(())
}
