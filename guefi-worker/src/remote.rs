use tokio::sync::{Mutex, MutexGuard};
use std::{env::current_exe, ffi::OsString, ops::{Deref, DerefMut}};

use crate::worker::message::{Command, Message, Response};
use guefi_lib::system::System;
use ipc_rpc::IpcRpc;

pub struct Remote {
    connection: IpcRpc<Message>,
}

static REMOTE_INSTANCE: Mutex<Option<Remote>> = Mutex::const_new(None);

struct RemoteGuard<'a> {
    guard: MutexGuard<'a, Option<Remote>>
}

impl Deref for RemoteGuard<'_> {
    type Target = Remote;
    fn deref(&self) -> &Remote {
        self.guard.as_ref().unwrap()
    }
}

impl DerefMut for RemoteGuard<'_> {
    fn deref_mut(&mut self) -> &mut Remote {
        self.guard.as_mut().unwrap()
    }
}

impl System for Remote {
    async fn get_boot_entries() -> Result<Vec<String>, String> {
        let rem = Self::get_instance().await;
        let response = rem.connection.send(Message::Command(Command::GetBootEntries)).await;
        match response {
            Ok(Message::Response(Response::GetBootEntries(result))) => return result,
            _ => panic!()
        }
    }
}

fn worker_executable() -> String {
    let exe = current_exe().unwrap();
    let path = exe.parent().unwrap();
    format!("{}/guefi-worker", path.to_string_lossy())
}

impl Remote {
    async fn get_instance<'a>() -> RemoteGuard<'a> {
        let mut guard = REMOTE_INSTANCE.lock().await;
        if guard.is_none() {
            *guard = Some(Remote::new().await);
        }
        RemoteGuard { guard }
    }

    async fn message_handler(_message: Message) -> Option<Message> { 
        None
    }

    async fn new() -> Remote {
        let worker_exe = worker_executable();
        let mut server_client_combo = IpcRpc::build()
            .finish(
                "pkexec",
                Self::message_handler,
                |key, cmd| {
                    let key: OsString = key.into();
                    cmd.arg(worker_exe);
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

    fn dispose() {
        let mut guard = REMOTE_INSTANCE.blocking_lock();
        *guard = None;
    }
}