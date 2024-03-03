use guefi_lib::get_entries;
use iced::{widget::{scrollable, text, column}, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {

}

pub struct BootPage {

}

impl BootPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) {

    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let entries = get_entries();
        let items: Vec<Element<_>> = entries.iter().map(|item| text(item).into()).collect();
        column(items).spacing(5).padding(5).into()
    }
}