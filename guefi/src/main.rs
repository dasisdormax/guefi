use window::MainWindow;

mod pages;
mod status;
mod window;
use guefi_worker::remote;

fn main() -> iced::Result {
    MainWindow::run_app()
}