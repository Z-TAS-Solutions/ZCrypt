use ZCrypt::zproto::PingRequest;
use ZCrypt::zproto::test_service_client::TestServiceClient;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| UnixStream::connect("/tmp/zproto.sock")))
        .await?;

    let mut client = TestServiceClient::new(channel);

    let request = tonic::Request::new(PingRequest {
        message: "Ping from Rust!".into(),
    });

    let response = client.ping(request).await?;

    println!("RESPONSE={:?}", response.into_inner().reply);

    Ok(())
}
