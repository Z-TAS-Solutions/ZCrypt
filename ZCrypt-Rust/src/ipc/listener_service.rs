#[cfg(windows)]
pub mod listener_service {
    use super::super::async_ipc::async_ipc_tokio::AsyncClient;
    use std::time::Duration;
    use tokio::task::JoinHandle;

    pub fn ipc_listener(pipe_path: String) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut ipc_client = AsyncClient::initialize(&pipe_path);

            loop {
                println!(
                    "Z-IPC: Attempting to connect to IPC pipe at {}...",
                    pipe_path
                );

                match ipc_client.connect_ex(2, 500).await {
                    Ok(_) => {
                        println!("Bridge: Connected to IPC! Starting message loop.");

                        loop {
                            match ipc_client.read().await {
                                Ok(buffer) => {
                                    // Gotta add the grpc bit here later
                                    println!("Received : {}", String::from_utf8_lossy(&buffer))
                                }
                                Err(e) => {
                                    // server died, prolly. but hey, we're rebooting.
                                    println!("Bridge: IPC read failed (Disconnected): {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // retrying the damned connection
                        tokio::time::sleep(Duration::from_millis(501)).await;
                    }
                }
            }
        })
    }
}
