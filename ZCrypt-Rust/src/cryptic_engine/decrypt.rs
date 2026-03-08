use super::aad_builder::AAD;
use super::cryptic_record::CrypticRecord;

use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit, Payload, generic_array::GenericArray},
};

pub fn gcm_open(kek: &[u8], record: CrypticRecord) -> Result<Vec<u8>, aes_gcm::Error> {
    let kek_cipher = Aes256Gcm::new(GenericArray::from_slice(kek));

    let aad = AAD::from_record(&record).to_bytes();
    let wrap_nonce_ = GenericArray::from_slice(&record.wrap_nonce);
    let template_nonce_ = GenericArray::from_slice(&record.template_nonce);

    let wrap_payload = Payload {
        msg: &record.wrapped_dek,
        aad: &aad,
    };

    let unwrapped_dek = kek_cipher.decrypt(wrap_nonce_, wrap_payload)?;

    let dek_cipher = Aes256Gcm::new(GenericArray::from_slice(&unwrapped_dek));

    let data_payload = Payload {
        msg: &record.ciphertext,
        aad: &aad,
    };

    let plaintext = dek_cipher.decrypt(template_nonce_, data_payload)?;

    return Ok(plaintext);
}
