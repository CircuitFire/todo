use frames::*; 
use crossterm::event::{KeyCode};

pub trait Display {
    fn display(&self) -> String;
}

impl Display for KeyCode {
    fn display(&self) -> String {
        match self {
            KeyCode::Backspace       => String::from("Backspace"),
            KeyCode::BackTab         => String::from("BackTab"),
            KeyCode::Char(character) => character.display(),
            KeyCode::Delete          => String::from("Delete"),
            KeyCode::Down            => String::from("Down"),
            KeyCode::End             => String::from("End"),
            KeyCode::Enter           => String::from("Enter"),
            KeyCode::Esc             => String::from("Esc"),
            KeyCode::F(num)          => format!("F{}", num),
            KeyCode::Home            => String::from("Home"),
            KeyCode::Insert          => String::from("Insert"),
            KeyCode::Left            => String::from("Left"),
            KeyCode::Null            => String::from("Null"),
            KeyCode::PageDown        => String::from("PageDown"),
            KeyCode::PageUp          => String::from("PageUp"),
            KeyCode::Right           => String::from("Right"),
            KeyCode::Tab             => String::from("Tab"),
            KeyCode::Up              => String::from("Up"),
        }
    }
}

impl Display for char {
    fn display(&self) -> String {
        match self {
            ' ' => String::from("Space"),
            ',' => String::from(",(<)"),
            '.' => String::from(".(>)"),
            _ => self.to_string(),
        }
    }
}

impl Display for Color {
    fn display(&self) -> String {
        match self {
            Color::Rgb{r, g, b} => format!("r:{:>3}, g:{:>3}, b:{:>3}", r, g, b),
            _ => String::new(),
        }
    }
}