#[cfg(unix)]

pub mod ipc {
    use std::io::{Read, Write};
    use std::os::unix::net::{UnixListener, UnixStream};

    pub struct Server {
        listener: UnixListener,
    }

    impl Server {
        pub fn bind(path: &str) -> std::io::Result<Self> {
            let _ = fs::remove_file(path);
            let listener = UnixListener::bind(path)?;
            Ok(Server { listener })
        }

        pub fn accept(&self) -> std::io::Result<UnixStream> {
            let (stream, _addr) = self.listener.accept()?;
            stream.write_all("Kneel Down Peasents")?;
            println!("Peasant Connected !");
            Ok(stream)
        }
    }

    pub type Client = UnixStream;

    pub fn connect(path: &str) -> std::io::Result<Client> {
        let stream = UnixStream::connect(path)?;
        println!("Connected");
        Ok(stream)
    }
}

#[cfg(windows)]
pub mod ipc {
    use named_pipe::{PipeClient, PipeOptions, PipeServer};
    use std::io::{Read, Write};

    pub struct Server {
        named_pipe: String,
        stream: Option<PipeServer>,
    }

    impl Server {
        pub fn initialize(path: &str) -> std::io::Result<Self> {
            Ok(Server {
                named_pipe: path.to_string(),
                stream: None,
            })
        }

        pub fn connect(&mut self) -> std::io::Result<()> {
            let server = PipeOptions::new(&self.named_pipe).single()?;
            println!("Server waiting for Peasant...");
            self.stream.insert(server.wait()?);
            println!("Peasant connected!");
            Ok(())
        }

        pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
            let mut buffer = vec![0u8; 1024];
            let n = self.stream.as_mut().unwrap().read(&mut buffer)?;
            buffer.truncate(n);
            Ok(buffer)
        }

        pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
            if let Some(stream) = self.stream.as_mut() {
                stream.write_all(data)?;
            }
            Ok(())
        }
    }

    pub struct Client {
        named_pipe: String,
        client: Option<PipeClient>,
    }

    impl Client {
        pub fn initialize(path: &str) -> std::io::Result<Self> {
            Ok(Client {
                named_pipe: path.to_string(),
                client: None,
            })
        }

        pub fn connect(&mut self) -> std::io::Result<()> {
            self.client.insert(PipeClient::connect(&self.named_pipe)?);
            Ok(())
        }

        pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
            if let Some(client) = self.client.as_mut() {
                client.write_all(data)?;
            }
            Ok(())
        }

        pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
            let mut buffer = vec![0u8; 1024];
            let n = self.client.as_mut().unwrap().read(&mut buffer)?;
            Ok(buffer)
        }
    }
}
