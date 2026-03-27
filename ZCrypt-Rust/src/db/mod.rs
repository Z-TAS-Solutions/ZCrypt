use sqlx::{PgPool, Error as SqlxError, postgres::PgPoolOptions, Row};
use crate::cryptic_engine::cryptic_record::CrypticRecord;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn init(db_url: &str) -> Result<Self, SqlxError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
            
        Ok(Self { pool })
    }

    pub async fn request_cryptic_record(&self, user_id: &str) -> Result<Option<CrypticRecord>, SqlxError> {
        
        let row = sqlx::query(
            "SELECT cryptic_record FROM users WHERE custom_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(r) = row {
            
            let record: sqlx::types::Json<CrypticRecord> = r.try_get("cryptic_record")?;
            return Ok(Some(record.0));
        }

        Ok(None)
    }
}
