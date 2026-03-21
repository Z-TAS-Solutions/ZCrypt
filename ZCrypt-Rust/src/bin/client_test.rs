use ZCrypt::ipc::ipc::ipc::connect;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let path = "rust_sockets.sock";
    let mut client = connect(path)?;
    client.write_all(b"Hellow There !")?;
    println!("message sent");
    Ok(())
}
