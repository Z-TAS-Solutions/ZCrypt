#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = ZCrypt::ipc::tonic_ipc::create_ipc_channel().await?;
    let mut client = ZCrypt::zproto::zproto::ping_service_client::PingServiceClient::new(channel);

    loop {
        let request = tonic::Request::new(ZCrypt::zproto::zproto::PingRequest {
            message: "Ping from Rust IPC!".into(),
        });

        match client.ping(request).await {
            Ok(response) => println!("response = {:?}", response.into_inner().reply),
            Err(e) => eprintln!("Error pinging service: {:?}", e),
        }

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
