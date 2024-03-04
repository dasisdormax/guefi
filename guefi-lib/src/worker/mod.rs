use std::marker::PhantomData;

use crate::system::System;

pub mod message;

use message::{Message, Command, Response};

pub struct Worker<T: System> {
    sys: PhantomData<T>,
}

impl<T: System> Worker<T> {
    pub fn new() -> Self {
        Self {
            sys: PhantomData
        }
    }

    pub async fn handle_command(&self, message: Message) -> Option<Message> {
        Self::handler(message).await
    }

    pub async fn handler(message: Message) -> Option<Message> {
        let Message::Command(cmd) = message else { return None };
        let response = match cmd {
            Command::GetBootEntries => Response::GetBootEntries(T::get_boot_entries().await),
            _ => return None,
        };
        Some(Message::Response(response))
    }
}