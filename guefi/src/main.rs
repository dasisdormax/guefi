mod pages;
mod status;

use guefi_lib::{add, get_entries};
use iced::{futures::FutureExt, widget::{button, checkbox, column, container, row, scrollable, text, vertical_space}, window::{self, Action}, Command, Element, Length, Sandbox, Settings, Theme};
use pages::boot::BootPage;
use status::StatusPanel;

#[derive(Debug, Clone)]
enum Message {
    SwitchPage(Page),
    Boot(pages::boot::Message),
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    Boot,
    Security,
    Encryption
}

struct MainWindow {
    current_page: Page,
    boot_page: BootPage,
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        MainWindow {
            current_page: Page::Boot,
            boot_page: BootPage::new(),
        }
    }

    fn title(&self) -> String {
        String::from("guEFI")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SwitchPage(page) => self.current_page = page,
            Message::Boot(msg) => self.boot_page.update(msg),
        };
    }

    fn view(&self) -> Element<Self::Message> {
        let content = match &self.current_page {
            Page::Boot => self.boot_page.view().map(Message::Boot),
            page => text(format!("{:?}", page)).into()
        };

        let page = scrollable(content).width(Length::Fill).height(Length::Fill).into();

        let tab = |text: &'static str, page: Page| {
            let disabled = self.current_page == page;
            button(text).on_press_maybe(Some(Message::SwitchPage(page)).filter(|_| !disabled))
        };

        let tab_row = row!(
            tab("Boot Entries", Page::Boot),
            tab("Security", Page::Security),
            tab("Encryption", Page::Encryption)
        );

        let mut cols: Vec<Element<_>> = vec![
            tab_row.into(),
            page,
            StatusPanel::view()
        ];
        column(cols).spacing(10).padding(10).into()
    }
}

fn main() -> iced::Result {
    MainWindow::run(Settings::default())
}
