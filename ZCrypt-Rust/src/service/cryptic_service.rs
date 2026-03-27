use crate::zproto::zproto::cryptic_service_server::CrypticService;
use crate::zproto::zproto::{FetchTemplateRequest, FetchTemplateResponse};
use tonic::{Request, Response, Status};
use std::env;

use crate::db::Database;
use crate::cryptic_engine::decrypt::gcm_open;

pub struct ZCrypticService {
    db: Database,
}

impl ZCrypticService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl CrypticService for ZCrypticService {
    async fn fetch_decrypted_template(
        &self,
        request: Request<FetchTemplateRequest>,
    ) -> Result<Response<FetchTemplateResponse>, Status> {
        let req = request.into_inner();
        println!("Received fetch template request for user: {}", req.user_id);

        // zeros just for testing, removing later..
        let kek_hex = env::var("KEK_SECRET").unwrap_or_else(|_| "0000000000000000000000000000000000000000000000000000000000000000".to_string());
        let kek_bytes = hex::decode(&kek_hex).map_err(|e| Status::internal(format!("Invalid KEK format: {}", e)))?;

        match self.db.request_cryptic_record(&req.user_id).await {
            Ok(Some(record)) => {
                match gcm_open(&kek_bytes, record) {
                    Ok(plaintext) => {
                        let response = FetchTemplateResponse {
                            success: true,
                            template_data: plaintext,
                            error_message: String::new(),
                        };
                        Ok(Response::new(response))
                    }
                    Err(e) => {
                        let response = FetchTemplateResponse {
                            success: false,
                            template_data: vec![],
                            error_message: format!("Decryption failed: {:?}", e),
                        };
                        Ok(Response::new(response))
                    }
                }
            }
            Ok(None) => {
                let response = FetchTemplateResponse {
                    success: false,
                    template_data: vec![],
                    error_message: format!("No record found for user {}", req.user_id),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn store_encrypted_template(
        &self,
        request: Request<crate::zproto::zproto::StoreTemplateRequest>,
    ) -> Result<Response<crate::zproto::zproto::StoreTemplateResponse>, Status> {
        let req = request.into_inner();
        println!("Received store template request for user: {}", req.user_id);

        let kek_hex = env::var("KEK_SECRET").unwrap_or_else(|_| {
            "0000000000000000000000000000000000000000000000000000000000000000".to_string()
        });
        let kek_bytes = hex::decode(&kek_hex)
            .map_err(|e| Status::internal(format!("Invalid KEK format: {}", e)))?;

        let aad = crate::cryptic_engine::aad_builder::AADBuilder::new()
            .user_id(&req.user_id)
            .template_id(&req.template_id)
            .template_type(&req.template_type)
            .schema_version(1)
            .template_ver(1)
            .build();

        let record = crate::cryptic_engine::encrypt::gcm_seal(&kek_bytes, aad, req.raw_template_data)
            .map_err(|e| Status::internal(format!("Encryption failed: {:?}", e)))?;

        match self.db.store_user_cryptic_record(&req.user_id, record).await {
            Ok(_) => {
                let response = crate::zproto::zproto::StoreTemplateResponse {
                    success: true,
                    error_message: String::new(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Database write error: {}", e))),
        }
    }
}
