use ZCrypt::ipc::ipc::ipc::Server;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let path = "rust_sockets.sock";

    let server = Server::bind(path)?;
    println!("Server running at {}", path);

    let mut stream = server.accept()?;
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer)?;
    println!("msg received: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
