use ZCrypt::ipc::tonic_ipc::create_ipc_channel;
use ZCrypt::zproto::zproto::PingRequest;
use ZCrypt::zproto::zproto::ping_service_client::PingServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = create_ipc_channel().await?;

    let mut client = PingServiceClient::new(channel);

    let request = tonic::Request::new(PingRequest {
        message: "Ping from Rust!".into(),
    });

    let response = client.ping(request).await?;

    println!("response = {:?}", response.into_inner().reply);

    Ok(())
}
