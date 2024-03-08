use std::marker::PhantomData;

use guefi_lib::system::{LocalSystem, System};
use guefi_worker::remote::Remote;
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use iced::widget::{button, column, row, scrollable, text};

use crate::pages::Page;
use crate::pages::boot::{self, BootPage};
use crate::status::StatusPanel;

#[derive(Debug, Clone)]
pub enum Message {
    SwitchPage(Page),
    Boot(boot::Message),
}

pub struct MainWindow<AppSystem: System = Remote> {
    current_page: Page,
    boot_page: BootPage,
    _sys: PhantomData<AppSystem>
}

impl<AppSystem: System> Application for MainWindow<AppSystem> {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            MainWindow {
                current_page: Page::Boot,
                boot_page: BootPage::new(),
                _sys: PhantomData
            },
            Command::perform(
                AppSystem::get_boot_entries(), 
                |result| Message::Boot(boot::Message::OnGetBootEntries(result))
            )
        )
    }

    fn title(&self) -> String {
        String::from("guEFI")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::SwitchPage(page) => {
                self.current_page = page
            },
            Message::Boot(msg) => self.boot_page.update(msg),
        };
        Command::none()
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

        let cols: Vec<Element<_>> = vec![
            tab_row.into(),
            page,
            StatusPanel::view()
        ];
        column(cols).spacing(10).padding(10).into()
    }
}

impl MainWindow {
    pub fn run_app() -> iced::Result {
        Self::run(Settings::default())
    }
}