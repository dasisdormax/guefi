use guefi_lib::{add, get_entries};
use iced::{futures::FutureExt, widget::{button, checkbox, column, container, scrollable, text, vertical_space}, window::{self, Action}, Command, Element, Length, Sandbox, Settings, Theme};

#[derive(Debug, Clone)]
enum Message {
    AddOne,
    AddText(String),
    Clear
}

#[derive(Default)]
struct MainWindow {
    messages: Vec<String>
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        MainWindow::default()
    }

    fn title(&self) -> String {
        String::from("guEFI")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Clear => self.messages.clear(),
            Message::AddOne => {
                let msg = Message::AddText(format!("{}", self.messages.len() + 1));
                self.update(msg);
            }
            Message::AddText(text) => self.messages.push(text)
        };
    }

    fn view(&self) -> Element<Self::Message> {
        let entries = get_entries();
        let mut cols: Vec<Element<_>> = vec![
            text("guEFI").into(),
            checkbox("Check!", false).into(),
            button("AddOne").on_press(Message::AddOne).into(),
            button("Clear").on_press(Message::Clear).into()
        ];
        let items: Vec<Element<_>> = entries.iter().map(|item| text(item).into()).collect();
        //cols.splice(2..2, items);
        let item_list = scrollable(
            column(items).spacing(5).padding(5)
        ).width(Length::Fill).height(Length::Fill).into();
        cols.insert(2, item_list);
        column(cols).spacing(10).padding(10).into()
    }
}

fn main() -> iced::Result {
    MainWindow::run(Settings::default())
}
