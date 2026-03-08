mod cryptic_engine;

use crate::cryptic_engine::aad_builder::AAD;
use crate::cryptic_engine::cryptic_record::CrypticRecord;
use crate::cryptic_engine::cryptic_record::CrypticRecordBuilder;
use crate::cryptic_engine::decrypt::gcm_open;
use crate::cryptic_engine::encrypt::gcm_seal;
use hex;

use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload, generic_array::GenericArray},
};

fn main() -> Result<(), aes_gcm::Error> {
    let aad_record = AAD {
        schema_version: 999,
        user_id: "zischl".to_string(),
        template_id: "534276".to_string(),
        template_type: "fusion".to_string(),
        template_ver: 1,
    };

    let kek = Aes256Gcm::generate_key(OsRng);

    let record = gcm_seal(&kek, aad_record, "hellow there !".into())?;
    println!("Cipher Text: {:#?}", record.ciphertext);

    let text = gcm_open(&kek, record);
    println!("Decrypted Text: {:#?}", text);

    Ok(())
}
