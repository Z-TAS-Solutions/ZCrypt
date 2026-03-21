use named_pipe::PipeClient;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let pipe_name = r"\\.\pipe\zcrypt_sock";

    let mut client = PipeClient::connect(pipe_name)?;
    println!("Connected to server!");

    client.write_all(b"Hello Peasant!")?;

    let mut buf = [0u8; 1024];
    let n = client.read(&mut buf)?;
    println!("response : {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
