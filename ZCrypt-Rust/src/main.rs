mod cryptic_engine;

use crate::cryptic_engine::aad_builder::AAD;
use crate::cryptic_engine::cryptic_record::CrypticRecord;
use crate::cryptic_engine::cryptic_record::CrypticRecordBuilder;
use hex;

fn main() -> Result<(), aes_gcm::Error> {
    use aes_gcm::{
        Aes256Gcm,
        aead::{Aead, AeadCore, KeyInit, OsRng, Payload},
    };

    let aad_record = AAD {
        schema_version: 999,
        user_id: "zischl".to_string(),
        template_id: "534276".to_string(),
        template_type: "fusion".to_string(),
        template_ver: 1,
    };

    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let template_nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let enc_payload = Payload {
        msg: b"hellow there !",
        aad: &aad_record.to_bytes(),
    };

    let ciphertext = cipher.encrypt(&template_nonce, enc_payload)?;

    println!("ciphertext : {}", hex::encode(&ciphertext));

    let dec_payload = Payload {
        msg: &ciphertext,
        aad: &aad_record.to_bytes(),
    };

    let plaintext = cipher.decrypt(&template_nonce, dec_payload)?;

    println!("plaintext: {}", std::str::from_utf8(&plaintext).unwrap());

    assert_eq!(&plaintext, b"hellow there !");

    Ok(())
}
