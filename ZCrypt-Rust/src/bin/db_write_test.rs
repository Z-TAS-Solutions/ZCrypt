use ZCrypt::db::Database;
use ZCrypt::cryptic_engine::cryptic_record::CrypticRecordBuilder;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing. Please set it in .env to test DB.");
    println!("Connecting to PG Database...");
    
    let db = Database::init(&db_url).await.expect("Failed to bind to postgres backend");
    println!("Successfully connected!");

    println!("Building dummy CrypticRecord...");
    
    let record = CrypticRecordBuilder::new()
        .user_id("468435165")
        .template_id("51351684")
        .template_type("FUSION")
        .template_ver(1)
        .template_nonce([1u8; 12])
        .wrap_nonce([2u8; 12])
        .wrapped_dek(vec![0; 32])
        .ciphertext(vec![1, 2, 3, 4, 5])
        .build();

    
    let target_custom_id = "468435165";

    println!("Executing UPDATE for user: {}", target_custom_id);
    match db.store_user_cryptic_record(target_custom_id, record).await {
        Ok(_) => {
            println!("Successfully executed the update query for {}!", target_custom_id);
            println!("If zero rows were updated, make sure the user exists and the 'cryptic_record' column is JSONB.");
        }
        Err(e) => {
            eprintln!("Database write test failed: {:?}", e);
        }
    }

    Ok(())
}
