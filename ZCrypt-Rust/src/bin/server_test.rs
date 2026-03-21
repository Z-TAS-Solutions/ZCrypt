use named_pipe::PipeOptions;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let path = r"\\.\pipe\zcrypt_sock";

    let server = PipeOptions::new(path).single()?;
    println!("Server waiting for Peasant...");
    let mut stream = server.wait()?;
    println!("Peasant connected!");

    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;
    println!("Response: {}", String::from_utf8_lossy(&buf[..n]));

    stream.write_all(b"Kneel Down Peasant !")?;
    Ok(())
}
