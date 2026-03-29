use std::time::Duration;
use tokio::time::sleep;
use tonic::transport::{Channel, Endpoint};
use tower::util::service_fn;

pub mod tonic_ipc {
    #[cfg(unix)]
    pub use tokio::net::UnixStream as IPCStream;

    #[cfg(windows)]
    pub use tokio::net::windows::named_pipe::NamedPipeClient as IPCStream;

    pub async fn connect() -> Result<IPCStream, std::io::Error> {
        #[cfg(unix)]
        {
            tokio::net::UnixStream::connect("/tmp/zpipcproto.sock").await
        }
        #[cfg(windows)]
        {
            use tokio::net::windows::named_pipe::ClientOptions;
            ClientOptions::new().open(r"\\.\pipe\zpipcproto")
        }
    }
}

pub async fn create_ipc_channel() -> Result<Channel, Box<dyn std::error::Error>> {
    loop {
        match Endpoint::from_static("http://Z-IPC")
            .connect_with_connector(service_fn(|_| tonic_ipc::connect()))
            .await
        {
            Ok(channel) => {
                println!("Connected to IPC!");
                return Ok(channel);
            }
            Err(_) => {
                println!("Go Hub not found, retrying in 1s...");
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

pub mod tonic_ipc_listener {
    use futures::stream::Stream;
    use std::pin::Pin;
    use tokio_stream::wrappers::ReceiverStream;

    #[cfg(unix)]
    use tokio::net::{UnixListener, UnixStream};

    #[cfg(windows)]
    use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};

    #[cfg(unix)]
    pub type IPCStream = UnixStream;

    #[cfg(windows)]
    pub type IPCStream = ServerIPCStream;

    pub async fn listener() -> Pin<Box<dyn Stream<Item = Result<IPCStream, std::io::Error>> + Send>>
    {
        #[cfg(unix)]
        {
            let path = "/tmp/zpipcproto.sock";
            let _ = std::fs::remove_file(path);
            let listener = UnixListener::bind(path).unwrap();
            Box::pin(
                tokio_stream::wrappers::UnixListenerStream::new(listener).map(|s| s.map_err(|e| e)),
            )
        }

        #[cfg(windows)]
        {
            let stream = futures::stream::unfold(true, |is_first| async move {
                let pipe_name = r"\\.\pipe\zpipcproto";
                let server = match ServerOptions::new()
                    .first_pipe_instance(is_first)
                    .create(pipe_name)
                {
                    Ok(s) => s,
                    Err(e) => return Some((Err(e), false)),
                };

                if let Err(e) = server.connect().await {
                    return Some((Err(e), false));
                }

                Some((Ok(ServerIPCStream(server)), false))
            });
            Box::pin(stream)
        }
    }

    #[cfg(windows)]
    pub struct ServerIPCStream(pub NamedPipeServer);

    #[cfg(windows)]
    impl tonic::transport::server::Connected for ServerIPCStream {
        type ConnectInfo = ();
        fn connect_info(&self) -> Self::ConnectInfo {
            ()
        }
    }

    #[cfg(windows)]
    impl tokio::io::AsyncRead for ServerIPCStream {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::pin::Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    #[cfg(windows)]
    impl tokio::io::AsyncWrite for ServerIPCStream {
        fn poll_write(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            std::pin::Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::pin::Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::pin::Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}

