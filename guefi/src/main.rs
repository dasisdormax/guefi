use window::MainWindow;

mod pages;
mod status;
mod window;
mod remote;

fn main() -> iced::Result {
    MainWindow::run_app()
}