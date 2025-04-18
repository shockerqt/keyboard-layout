use iced::{
    Element,
    widget::{button, column},
};

#[derive(Default)]
pub struct KeyboardLayout;

#[derive(Clone, Debug)]
pub enum Message {
    KeyA,
    KeyS,
}

impl KeyboardLayout {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::KeyA => {
                println!("Key A");
            }
            Message::KeyS => {
                println!("Key S");
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![button("A").on_press(Message::KeyA)].into()
    }
}
