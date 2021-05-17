use frames::crossterm::event::{KeyCode};
use super::buffer::*;
use std::io::prelude::*;
use std::fs::File;

use super::display::Display;

use std::fs::OpenOptions;

pub enum Control{
    Up,
    Down,
    Left,
    Right,
    Select,
    Esc,
    Del,
    Complete,
    PrintAll,
    PrintUnfinished,
    SetRoot,
    BackRoot,
    Save,
    Edit,
    Config,
    None,
}

impl Control {
    pub fn default(&self) -> KeyCode {
        match self {
            Control::Up              => KeyCode::Up,
            Control::Down            => KeyCode::Down,
            Control::Left            => KeyCode::Left,
            Control::Right           => KeyCode::Right,
            Control::Select          => KeyCode::Enter,
            Control::Esc             => KeyCode::Esc,
            Control::Del             => KeyCode::Delete,
            Control::Complete        => KeyCode::Char(' '),
            Control::PrintAll        => KeyCode::Char('p'),
            Control::PrintUnfinished => KeyCode::Char('o'),
            Control::SetRoot         => KeyCode::Char('.'),
            Control::BackRoot        => KeyCode::Char(','),
            Control::Save            => KeyCode::Char('s'),
            Control::Edit            => KeyCode::Char('e'),
            Control::Config          => KeyCode::Char('c'),
            Control::None            => KeyCode::Null,
        }
    }

    pub fn from_num(num: usize) -> Control{
        match num {
            0  => Control::Up,
            1  => Control::Down,
            2  => Control::Left,
            3  => Control::Right,
            4  => Control::Select,
            5  => Control::Esc,
            6  => Control::Del,
            7  => Control::Complete,
            8  => Control::PrintAll,
            9  => Control::PrintUnfinished,
            10 => Control::SetRoot,
            11 => Control::BackRoot,
            12 => Control::Save,
            13 => Control::Edit,
            14 => Control::Config,
            _  => Control::None,
        }
    }

    pub fn string(&self) -> &str {
        match self {
            Control::Up              => "Move up",
            Control::Down            => "Move down",
            Control::Left            => "Decrease depth",
            Control::Right           => "Increase depth",
            Control::Select          => "Select/add task",
            Control::Esc             => "Quit/back",
            Control::Del             => "Delete/reset",
            Control::Complete        => "Toggle completed",
            Control::PrintAll        => "Save list to txt file",
            Control::PrintUnfinished => "Save unfinished to txt file",
            Control::SetRoot         => "Set entry focus",
            Control::BackRoot        => "Return focus",
            Control::Save            => "Save list",
            Control::Edit            => "Edit entry",
            Control::Config          => "Open/Close config menu",
            Control::None            => "Unused control",
        }
    }
}

pub struct ControlSettings {
    keys: [KeyCode; 15],
}

impl ControlSettings {
    pub fn new() -> Result<ControlSettings, std::io::Error> {
        if let Ok(mut file) = File::open("todo.cnf"){
            file.seek(std::io::SeekFrom::Start(6))?;
            let mut buffer = [0; 5];
            

            let mut temp = [KeyCode::Null; 15];

            for i in &mut temp {
                file.read(&mut buffer)?;
                *i = buff_to_keycode(&buffer).unwrap();
            }

            Ok(ControlSettings {
                keys: temp,
            })
        }
        else {
            Ok(ControlSettings {
                keys: [
                    Control::Up.default(),
                    Control::Down.default(),
                    Control::Left.default(),
                    Control::Right.default(),
                    Control::Select.default(),
                    Control::Esc.default(),
                    Control::Del.default(),
                    Control::Complete.default(),
                    Control::PrintAll.default(),
                    Control::PrintUnfinished.default(),
                    Control::SetRoot.default(),
                    Control::BackRoot.default(),
                    Control::Save.default(),
                    Control::Edit.default(),
                    Control::Config.default(),
                ]
            })
        }
    }

    pub fn save(&self) -> std::io::Result<()>{
        let mut file =  OpenOptions::new().write(true).truncate(false).create(true).open("todo.cnf")?;
        file.seek(std::io::SeekFrom::Start(6))?;
        let mut buffer = [0; 5];

        for i in &self.keys {
            i.into_buffer(&mut buffer);
            file.write(&buffer)?;
        }

        file.write(&buffer)?;
        Ok(())
    }

    pub fn keycode_to_control(&self, code: KeyCode) -> Control {
        match code {
            x if x == self.keys[0]  => Control::Up,
            x if x == self.keys[1]  => Control::Down,
            x if x == self.keys[2]  => Control::Left,
            x if x == self.keys[3]  => Control::Right,
            x if x == self.keys[4]  => Control::Select,
            x if x == self.keys[5]  => Control::Esc,
            x if x == self.keys[6]  => Control::Del,
            x if x == self.keys[7]  => Control::Complete,
            x if x == self.keys[8]  => Control::PrintAll,
            x if x == self.keys[9]  => Control::PrintUnfinished,
            x if x == self.keys[10] => Control::SetRoot,
            x if x == self.keys[11] => Control::BackRoot,
            x if x == self.keys[12] => Control::Save,
            x if x == self.keys[13] => Control::Edit,
            x if x == self.keys[14] => Control::Config,
            _ => Control::None,
        }
    }

    //pub fn key_at_num(&self, num: usize) -> KeyCode {
    //    self.keys[num]
    //}

    pub fn set_key_at_num(&mut self, num: usize, code: KeyCode) {
        self.keys[num] = code;
    }

    pub fn key_at_control(&self, control: Control) -> KeyCode {
        self.keys[control as usize]
    }

    //pub fn set_key_at_control(&mut self, control: Control, code: KeyCode) {
    //    self.keys[control as usize] = code;
    //}

    pub fn to_string(&self, num: usize) -> String {
        format!("{:<27}: {}", Control::from_num(num).string(), self.keys[num].display())
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}