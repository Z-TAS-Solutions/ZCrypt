pub mod ipc {
    use std::io::{Read, Write};
    use std::os::unix::net::{UnixListener, UnixStream};

    fn client() -> std::io::Result<()> {
        let socket_path = "/tmp/rust_sockets.sock";

        let _ = std::fs::remove_file(socket_path);

        let listener = UnixListener::bind(socket_path)?;

        println!("Server running.. {}", socket_path);

        if let Ok((mut stream, _addr)) = listener.accept() {
            println!("Peasent Connected !");

            let mut buffer = [0u8; 1024];
            let n = stream.read(&mut buffer)?;
            println!("Response : {}", String::from_utf8_lossy(&buffer[..n]));

            stream.write_all(b"Kneel Down Peasents")?;
        }

        Ok(())
    }

    fn server() -> std::io::Result<()> {
        let socket_path = "/tmp/rust_socketc.sock";

        let mut stream = UnixStream::connect(socket_path)?;
        println!("Connected");

        stream.write_all(b"Hellow There Peaseants")?;

        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer)?;
        println!("response : {}", String::from_utf8_lossy(&buffer[..n]));

        Ok(())
    }
}
