use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/rust_socket.sock";

    let mut stream = UnixStream::connect(socket_path)?;
    println!("Connected");

    stream.write_all(b"Hellow There Peaseants")?;

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer)?;
    println!("response : {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
