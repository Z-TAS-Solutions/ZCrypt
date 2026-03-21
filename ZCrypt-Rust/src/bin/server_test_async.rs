use ZCrypt::ipc::async_ipc::async_ipc_tokio::AsyncServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut server = AsyncServer::initialize(r"\\.\pipe\mypipe");

    server.connect().await?;

    let data = server.read().await?;
    println!("Received: {}", String::from_utf8_lossy(&data));

    server.write(b"bleh !").await?;

    Ok(())
}
