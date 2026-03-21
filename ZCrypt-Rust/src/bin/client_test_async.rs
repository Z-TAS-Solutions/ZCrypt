use ZCrypt::ipc::async_ipc::async_ipc_tokio::AsyncClient;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut client = AsyncClient::initialize(r"\\.\pipe\mypipe");

    // Use retry version (important for async startup timing)
    client.connect_ex(10, 500).await?;

    client.write(b"Hellow Peasant !!!").await?;

    let data = client.read().await?;
    println!("Received: {}", String::from_utf8_lossy(&data));

    Ok(())
}
