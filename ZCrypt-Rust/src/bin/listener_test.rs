use ZCrypt::ipc::listener_service::listener_service::ipc_listener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pipe_name = r"\\.\pipe\Z-IPC".to_string();

    let (listener_service, enqueue) = ipc_listener(pipe_name);

    println!("shit works ! :]");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
