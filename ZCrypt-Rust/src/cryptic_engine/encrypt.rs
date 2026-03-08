use super::aad_builder::AAD;
use super::cryptic_record::CrypticRecord;

use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload, generic_array::GenericArray},
};

pub fn gcm_seal(
    kek: &[u8],
    aad_record: AAD,
    data: Vec<u8>,
) -> Result<CrypticRecord, aes_gcm::Error> {
    let aad = aad_record.to_bytes();

    let data_payload = Payload {
        msg: &data,
        aad: &aad,
    };

    let dek = Aes256Gcm::generate_key(OsRng);
    let dek_cipher = Aes256Gcm::new(&dek);
    let template_nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher_text = dek_cipher.encrypt(&template_nonce, data_payload)?;

    let kek_cipher = Aes256Gcm::new(GenericArray::from_slice(kek));
    let wrap_nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let wrap_payload = Payload {
        msg: dek.as_slice(),
        aad: &aad,
    };

    let wrapped_dek = kek_cipher.encrypt(&wrap_nonce, wrap_payload)?;

    let record = CrypticRecord {
        template_nonce: template_nonce.into(),
        wrapped_dek,
        wrap_nonce: wrap_nonce.into(),
        ciphertext: cipher_text,
        ..aad_record.into()
    };

    return Ok(record);
}
