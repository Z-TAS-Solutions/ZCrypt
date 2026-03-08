mod cryptic_engine;

use crate::cryptic_engine::aad_builder::AAD;
use crate::cryptic_engine::cryptic_record::CrypticRecord;
use crate::cryptic_engine::cryptic_record::CrypticRecordBuilder;
use hex;

fn main() -> Result<(), aes_gcm::Error> {
    use aes_gcm::{
        Aes256Gcm,
        aead::{Aead, AeadCore, KeyInit, OsRng, Payload, generic_array::GenericArray},
    };

    let aad_record = AAD {
        schema_version: 999,
        user_id: "zischl".to_string(),
        template_id: "534276".to_string(),
        template_type: "fusion".to_string(),
        template_ver: 1,
    };

    let dek = Aes256Gcm::generate_key(OsRng);
    let dek_cipher = Aes256Gcm::new(&dek);
    let template_nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let enc_payload = Payload {
        msg: b"hellow there !",
        aad: &aad_record.to_bytes(),
    };

    let cipher_text = dek_cipher.encrypt(&template_nonce, enc_payload)?;

    let kek = Aes256Gcm::generate_key(OsRng);
    let kek_cipher = Aes256Gcm::new(&kek);
    let wrap_nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let enc_payload = Payload {
        msg: &dek,
        aad: &aad_record.to_bytes(),
    };

    let wrapped_dek = kek_cipher.encrypt(&wrap_nonce, enc_payload)?;

    println!("ciphertext : {}", hex::encode(&cipher_text));

    let record = CrypticRecord {
        schema_version: aad_record.schema_version,
        user_id: aad_record.user_id.clone(),
        template_id: aad_record.template_id.clone(),
        template_type: aad_record.template_type.clone(),
        template_ver: aad_record.template_ver,

        template_nonce: template_nonce.into(),
        wrapped_dek: wrapped_dek,
        wrap_nonce: wrap_nonce.into(),
        ciphertext: cipher_text,
    };

    let kek_cipher = Aes256Gcm::new(&kek);

    let dec_payload = Payload {
        msg: &record.wrapped_dek,
        aad: &aad_record.to_bytes(),
    };

    let unwrapped_dek = kek_cipher.decrypt(&record.wrap_nonce.into(), dec_payload)?;

    let dek_wcipher = Aes256Gcm::new(&GenericArray::from_slice(&unwrapped_dek));

    let dec_payload = Payload {
        msg: &record.ciphertext,
        aad: &aad_record.to_bytes(),
    };

    let plaintext = dek_wcipher.decrypt(&record.template_nonce.into(), dec_payload)?;

    println!("plaintext: {}", std::str::from_utf8(&plaintext).unwrap());

    assert_eq!(&plaintext, b"hellow there !");

    Ok(())
}
