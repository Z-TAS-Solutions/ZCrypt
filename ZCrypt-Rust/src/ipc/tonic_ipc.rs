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
            tokio::net::UnixStream::connect("/tmp/zproto.sock").await
        }
        #[cfg(windows)]
        {
            use tokio::net::windows::named_pipe::ClientOptions;
            ClientOptions::new().open(r"\\.\pipe\zproto")
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
    pub type IPCStream = NamedPipeServer;

    pub async fn listener() -> Pin<Box<dyn Stream<Item = Result<IPCStream, std::io::Error>> + Send>>
    {
        #[cfg(unix)]
        {
            let path = "/tmp/zproto.sock";
            let _ = std::fs::remove_file(path);
            let listener = UnixListener::bind(path).unwrap();
            Box::pin(
                tokio_stream::wrappers::UnixListenerStream::new(listener).map(|s| s.map_err(|e| e)),
            )
        }

        #[cfg(windows)]
        {
            let pipe_name = r"\\.\pipe\zproto";
            let server = ServerOptions::new().create(pipe_name).unwrap();
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            tx.send(Ok(server)).await.unwrap();
            Box::pin(ReceiverStream::new(rx))
        }
    }
}
