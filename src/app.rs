use crate::components::keyboard_layout::{self, KeyboardLayout};
use iced::{Element, Event, Subscription, Task, Theme, event, keyboard, widget::container, window};

#[derive(Default)]
pub struct App {
    layout: KeyboardLayout,
}

#[derive(Clone, Debug)]
pub enum Message {
    KeyboardLayout(keyboard_layout::Message),
    EventOccurred(Event),
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyboardLayout(message) => {
                self.layout.update(message);

                Task::none()
            }
            Message::EventOccurred(event) => match event {
                Event::Window(window::Event::CloseRequested) => {
                    window::get_latest().and_then(window::close)
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modified_key,
                    physical_key,
                    location,
                    modifiers,
                    text,
                }) => {
                    println!("Key pressed {:?}", text);

                    Task::none()
                }
                _ => Task::none(),
            },
        }
    }
    pub fn view(&self) -> Element<Message> {
        let layout_view = self.layout.view().map(Message::KeyboardLayout);
        container(layout_view).padding(10).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}
