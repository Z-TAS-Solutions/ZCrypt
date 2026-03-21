use ZCrypt::ipc::ipc::ipc::Server;

fn main() -> std::io::Result<()> {
    let mut server = Server::initialize(r"\\.\pipe\mypipe")?;

    server.connect()?;

    server.read()?;
    Ok(())
}
