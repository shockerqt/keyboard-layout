mod app;
mod components;

use app::App;

fn main() -> iced::Result {
    iced::run("A cool counter", App::update, App::view)
}
