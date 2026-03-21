use ZCrypt::ipc::ipc::ipc::Server;

fn main() -> std::io::Result<()> {
    let mut server = Server::initialize(r"\\.\pipe\mypipe")?;

    server.connect()?;
    let data = server.read()?;
    println!("Received: {}", String::from_utf8_lossy(&data));

    server.write("bleh !".as_bytes());

    Ok(())
}
