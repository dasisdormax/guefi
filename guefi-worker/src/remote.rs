use std::ffi::OsString;

use async_once_cell::OnceCell;

use crate::worker::message::Message;
use guefi_lib::system::System;
use ipc_rpc::IpcRpc;

pub struct Remote {
    connection: IpcRpc<Message>,
}

static REMOTE_INSTANCE: OnceCell<Remote> = OnceCell::new();

impl System for Remote {
    async fn get_boot_entries() -> Result<Vec<String>, String> {
        todo!()
    }
}

async fn message_handler(_message: Message) -> Option<Message> { 
    None
}

impl Remote {
    async fn get_instance() -> &'static Remote {
        REMOTE_INSTANCE.get_or_init(Self::new()).await
    }

    async fn new() -> Remote {
        let mut server_client_combo = IpcRpc::build()
            .finish(
                "target/debug/examples/client",
                message_handler,
                |key, cmd| {
                    let key: OsString = key.into();
                    cmd.arg(key);
                },
            )
            .await
            .unwrap();
        server_client_combo
            .server
            .wait_for_client_to_connect()
            .await
            .unwrap();
        Remote {
            connection: server_client_combo
        }
    }
}