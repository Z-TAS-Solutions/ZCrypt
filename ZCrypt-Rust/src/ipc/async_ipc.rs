#[cfg(windows)]
pub mod async_ipc_tokio {
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::windows::named_pipe::{
        ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions,
    };
    use tokio::time::sleep;

    pub struct AsyncServer {
        named_pipe: String,
        stream: Option<NamedPipeServer>,
    }

    impl AsyncServer {
        pub fn initialize(path: &str) -> Self {
            Self {
                named_pipe: path.to_string(),
                stream: None,
            }
        }

        pub async fn connect(&mut self) -> std::io::Result<()> {
            let server = ServerOptions::new()
                .first_pipe_instance(true)
                .create(&self.named_pipe)?;
            println!("Awaiting peasants !");
            server.connect().await?;
            println!("Async server: client connected!");
            self.stream = Some(server);
            Ok(())
        }

        pub async fn read(&mut self) -> std::io::Result<Vec<u8>> {
            let stream = self.stream.as_mut().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotConnected, "Server not connected")
            })?;

            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).await?;
            let len = u32::from_le_bytes(len_buf) as usize;

            let mut buf = vec![0u8; len];
            stream.read_exact(&mut buf).await?;
            Ok(buf)
        }

        pub async fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
            let stream = self.stream.as_mut().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotConnected, "Server not connected")
            })?;

            let len = data.len() as u32;
            stream.write_all(&len.to_le_bytes()).await?;
            stream.write_all(data).await?;
            stream.flush().await?;
            Ok(())
        }
    }

    pub struct AsyncClient {
        named_pipe: String,
        client: Option<NamedPipeClient>,
    }

    impl AsyncClient {
        pub fn initialize(path: &str) -> Self {
            Self {
                named_pipe: path.to_string(),
                client: None,
            }
        }

        pub async fn connect(&mut self) -> std::io::Result<()> {
            let client = ClientOptions::new().open(&self.named_pipe)?;
            self.client = Some(client);
            Ok(())
        }

        pub async fn connect_ex(&mut self, retries: u32, delay_ms: u64) -> std::io::Result<()> {
            for _ in 0..retries {
                match ClientOptions::new().open(&self.named_pipe) {
                    Ok(client) => {
                        self.client = Some(client);
                        return Ok(());
                    }
                    Err(_) => sleep(Duration::from_millis(delay_ms)).await,
                }
            }
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect",
            ))
        }

        pub async fn read(&mut self) -> std::io::Result<Vec<u8>> {
            let client = self.client.as_mut().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotConnected, "Client not connected")
            })?;

            let mut len_buf = [0u8; 4];
            client.read_exact(&mut len_buf).await?;
            let len = u32::from_le_bytes(len_buf) as usize;

            let mut buf = vec![0u8; len];
            client.read_exact(&mut buf).await?;
            Ok(buf)
        }

        pub async fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
            let client = self.client.as_mut().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotConnected, "Client not connected")
            })?;

            let len = data.len() as u32;
            client.write_all(&len.to_le_bytes()).await?;
            client.write_all(data).await?;
            client.flush().await?;
            Ok(())
        }
    }
}
