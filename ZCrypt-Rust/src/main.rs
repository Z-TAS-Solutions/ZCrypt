use ZCrypt::ipc::tonic_ipc::tonic_ipc_listener;
use ZCrypt::zproto::zproto::PingRequest;
use ZCrypt::zproto::zproto::ping_service_server::{PingService, PingServiceServer};
use ZCrypt::zproto::zproto::{self, PingResponse};
use tonic::transport::Server;

struct ZIPCPingService;

#[tonic::async_trait]
impl PingService for ZIPCPingService {
    async fn ping(
        &self,
        request: tonic::Request<PingRequest>,
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {
        let msg = request.into_inner().message;

        println!("Request : {}", msg);

        let reply = PingResponse {
            reply: format!("Ping : 0ms"),
        };

        Ok(tonic::Response::new(reply))
    }
}

use ZCrypt::zproto::zproto::cryptic_service_server::CrypticServiceServer;
use ZCrypt::service::cryptic_service::ZCrypticService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ping_svc = PingServiceServer::new(ZIPCPingService);
    let cryptic_svc = CrypticServiceServer::new(ZCrypticService);

    let incoming = tonic_ipc_listener::listener().await;

    println!("Starting IPC server...");
    Server::builder()
        .add_service(ping_svc)
        .add_service(cryptic_svc)
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}
