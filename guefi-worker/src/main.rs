use std::env;

use guefi_lib::worker::Worker;
use guefi_lib::system::SystemImpl;
use ipc_rpc::{ConnectionKey, IpcRpcClient};

#[tokio::main]
async fn main() -> Result<(), ()> {
    // TODO: Receive key through stdin
    let key = env::args().nth(1).expect("First argument is server key");

    let client =
        IpcRpcClient::initialize_client(
            ConnectionKey::try_from(key).unwrap(), 
            Worker::<SystemImpl>::handler
        ).await.map_err(|_| ())?;
    client.wait_for_server_to_disconnect().await.map_err(|_| ())?;
    Ok(())
}
