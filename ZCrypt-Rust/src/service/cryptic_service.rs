use crate::zpipcproto::zpipcproto::cryptic_service_server::CrypticService;
use crate::zpipcproto::zpipcproto::{FetchTemplateRequest, FetchTemplateResponse};
use std::env;
use tonic::{Request, Response, Status};

use crate::cryptic_engine::decrypt::gcm_open;
use crate::db::Database;

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

        let kek_hex = env::var("KEK_SECRET").unwrap();
        let kek_bytes = hex::decode(&kek_hex)
            .map_err(|e| Status::internal(format!("Invalid KEK format: {}", e)))?;

        match self.db.request_cryptic_record(&req.user_id).await {
            Ok(Some(record)) => match gcm_open(&kek_bytes, record) {
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
            },
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
        request: Request<crate::zpipcproto::zpipcproto::StoreTemplateRequest>,
    ) -> Result<Response<crate::zpipcproto::zpipcproto::StoreTemplateResponse>, Status> {
        let req = request.into_inner();
        println!("Received store template request for user: {}", req.user_id);

        let kek_hex = env::var("KEK_SECRET").unwrap();
        let kek_bytes = hex::decode(&kek_hex)
            .map_err(|e| Status::internal(format!("Invalid KEK format: {}", e)))?;

        let aad = crate::cryptic_engine::aad_builder::AADBuilder::new()
            .user_id(&req.user_id)
            .template_id(&req.template_id)
            .template_type(&req.template_type)
            .schema_version(1)
            .template_ver(1)
            .build();

        let record =
            crate::cryptic_engine::encrypt::gcm_seal(&kek_bytes, aad, req.raw_template_data)
                .map_err(|e| Status::internal(format!("Encryption failed: {:?}", e)))?;

        match self
            .db
            .store_user_cryptic_record(&req.user_id, record)
            .await
        {
            Ok(_) => {
                let response = crate::zpipcproto::zpipcproto::StoreTemplateResponse {
                    success: true,
                    error_message: String::new(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Database write error: {}", e))),
        }
    }

    async fn match_template(
        &self,
        request: Request<crate::zpipcproto::zpipcproto::MatchTemplateRequest>,
    ) -> Result<Response<crate::zpipcproto::zpipcproto::MatchTemplateResponse>, Status> {
        let req = request.into_inner();
        println!("Received match template request ");

        let kek_hex = env::var("KEK_SECRET").unwrap();
        let kek_bytes = hex::decode(&kek_hex)
            .map_err(|e| Status::internal(format!("Failed To Retrieve KEK: {}", e)))?;

        /*let record = match self.db.request_cryptic_record(&req.user_id).await {
            Ok(Some(rec)) => rec,
            Ok(None) => {
                return Ok(Response::new(
                    crate::zpipcproto::zpipcproto::MatchTemplateResponse {
                        is_match: false,
                        confidence_score: 0.0,
                        error_message: format!("User {} not found", req.user_id),
                    },
                ));
            }
            Err(e) => return Err(Status::internal(format!("Database error: {}", e))),
        };*/

        println!("Running Matching Engine !");

        /*let decrypted_db_template = match gcm_open(&kek_bytes, record) {
            Ok(plaintext) => plaintext,
            Err(e) => {
                return Ok(Response::new(
                    crate::zpipcproto::zpipcproto::MatchTemplateResponse {
                        is_match: false,
                        confidence_score: 0.0,
                        error_message: format!("Failed to decrypt DB template: {:?}", e),
                    },
                ));
            }
        };*/

        unsafe extern "C" {
            fn perform_template_match() -> bool;
        }

        let match_result = unsafe { perform_template_match() };
        println!("Match Result : {:?}", match_result);

        Ok(Response::new(
            crate::zpipcproto::zpipcproto::MatchTemplateResponse {
                is_match: match_result,
                confidence_score: 81.0,
                error_message: String::new(),
            },
        ))
    }
}
