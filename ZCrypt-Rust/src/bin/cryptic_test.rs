use ZCrypt::cryptic_engine;

use ZCrypt::cryptic_engine::aad_builder::AAD;
use ZCrypt::cryptic_engine::cryptic_record::CrypticRecord;
use ZCrypt::cryptic_engine::cryptic_record::CrypticRecordBuilder;
use ZCrypt::cryptic_engine::decrypt::gcm_open;
use ZCrypt::cryptic_engine::encrypt::gcm_seal;
use std::fs;

use aes_gcm::{
    Aes256Gcm,
    aead::{KeyInit, OsRng},
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

    let serialized = serde_json::to_string_pretty(&record).unwrap();
    let result = fs::write("test.json", serialized);

    let data = fs::read_to_string("test.json").unwrap();
    let deserialized: CrypticRecord = serde_json::from_str(&data).unwrap();

    let text = gcm_open(&kek, deserialized);
    println!("Decrypted Text: {:#?}", text);

    Ok(())
}
