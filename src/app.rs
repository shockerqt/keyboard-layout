use crate::components::keyboard_layout::{self, KeyboardLayout};
use iced::{Element, widget::container};

#[derive(Default)]
pub struct App {
    layout: KeyboardLayout,
}

#[derive(Clone, Debug)]
pub enum Message {
    KeyboardLayout(keyboard_layout::Message),
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::KeyboardLayout(message) => {
                self.layout.update(message);
            }
        }
    }
    pub fn view(&self) -> Element<Message> {
        let layout_view = self.layout.view().map(Message::KeyboardLayout);
        container(layout_view).padding(10).into()
    }
}
