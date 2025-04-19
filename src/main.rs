mod app;
mod components;
mod events;

use app::App;

fn main() -> iced::Result {
    iced::application("A cool counter", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}
