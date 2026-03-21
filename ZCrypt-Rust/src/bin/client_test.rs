use ZCrypt::ipc::ipc::ipc::Client;

fn main() -> std::io::Result<()> {
    let pipe_name = r"\\.\pipe\zcrypt_sock";

    let mut client = Client::initialize(r"\\.\pipe\mypipe")?;

    client.connect()?;

    client.write("Hellow Peasant !!!".as_bytes())?;

    let data = client.read()?;
    println!("Received: {}", String::from_utf8_lossy(&data));
    Ok(())
}
