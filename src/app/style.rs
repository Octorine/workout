use iced::theme::{self, Theme};
use iced::theme::{Container, Text};

pub struct AppStyle {
    pub header: Text,
    pub text: Text,
    pub page: Container,
    pub table_column_width: u16,
    pub table_row_padding: u16,
    pub exercise_row_padding: u16,
}

impl Default for AppStyle {
    fn default() -> Self {
        AppStyle {
            header: Text::default(),
            text: Text::default(),
            page: Container::default(),
            table_column_width: 200,
            table_row_padding: 10,
            exercise_row_padding: 10,
        }
    }
}
