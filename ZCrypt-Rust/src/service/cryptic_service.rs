use crate::zproto::zproto::cryptic_service_server::CrypticService;
use crate::zproto::zproto::{FetchTemplateRequest, FetchTemplateResponse};
use tonic::{Request, Response, Status};

pub struct ZCrypticService;

#[tonic::async_trait]
impl CrypticService for ZCrypticService {
    async fn fetch_decrypted_template(
        &self,
        request: Request<FetchTemplateRequest>,
    ) -> Result<Response<FetchTemplateResponse>, Status> {
        let req = request.into_inner();
        println!("Received fetch template request for user: {}", req.user_id);

        
        let mock_decrypted_payload = b"mock_decrypted_template_data".to_vec();

        let response = FetchTemplateResponse {
            success: true,
            template_data: mock_decrypted_payload,
            error_message: String::new(),
        };

        Ok(Response::new(response))
    }
}
