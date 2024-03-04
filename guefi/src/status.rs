use iced::{widget::{text, row}, Element};

pub struct StatusPanel {

}

pub enum Message {

}

impl StatusPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: Message) {

    }

    pub fn view<'a, T: 'a>() -> Element<'a, T> {
        row!(
            text(format!("guEFI version {}", env!("CARGO_PKG_VERSION")))
        ).into()
    }
}