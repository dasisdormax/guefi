use crate::remote::Remote;
use guefi_lib::system::System;
use iced::{widget::{text, column}, Element};

#[derive(Debug, Clone)]
pub enum Message {
    OnGetBootEntries(Result<Vec<String>, String>),
}

pub struct BootPage {
    entries: Vec<String>
}

impl BootPage {
    pub fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    pub fn update(&mut self, _message: Message) {
        match _message {
            Message::OnGetBootEntries(result) => {
                let entries = result.unwrap_or_else(|err| vec![err]);
                self.entries = entries
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let items: Vec<Element<_>> = self.entries.iter().map(|item| text(item).into()).collect();
        column(items).spacing(5).padding(5).into()
    }
}