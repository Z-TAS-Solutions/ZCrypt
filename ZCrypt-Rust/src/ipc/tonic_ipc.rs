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
    let channel = Endpoint::from_static("http://Z-IPC")
        .connect_with_connector(service_fn(|_| tonic_ipc::connect()))
        .await?;

    Ok(channel)
}
