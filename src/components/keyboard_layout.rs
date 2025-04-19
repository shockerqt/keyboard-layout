use iced::{
    Element,
    widget::{Column, Row, button, column, text},
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
        let mut rows = Column::new();

        for row_index in 0..3 {
            let mut col = Row::new();

            for col_index in 0..6 {
                col = col.push(text!("{col_index}, {row_index}"));
            }

            rows = rows.push(col);
        }

        rows.into()
    }
}
