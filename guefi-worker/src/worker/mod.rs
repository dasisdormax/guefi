use std::marker::PhantomData;

use guefi_lib::system::{LocalSystem, System};

pub(crate) mod message;

use ipc_rpc::{ConnectionKey, IpcRpcClient};
use message::{Message, Command, Response};

struct Worker<T: System> {
    sys: PhantomData<T>,
}

impl<T: System> Worker<T> {
    fn new() -> Self {
        Self {
            sys: PhantomData
        }
    }

    async fn handle_command(&self, message: Message) -> Option<Message> {
        Self::handler(message).await
    }

    async fn handler(message: Message) -> Option<Message> {
        let Message::Command(cmd) = message else { return None };
        let response = match cmd {
            Command::GetBootEntries => Response::GetBootEntries(T::get_boot_entries().await),
            _ => return None,
        };
        Some(Message::Response(response))
    }
}

pub async fn run_client(key: String) {
    let Ok(client) =
        IpcRpcClient::initialize_client(
            ConnectionKey::try_from(key).unwrap(), 
            Worker::<LocalSystem>::handler
        ).await else { return };
    
    let _ = client.wait_for_server_to_disconnect().await;
}