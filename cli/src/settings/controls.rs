use frames::{crossterm::event::*, Manager};
use crate::*;
use super::*;
use core::Position;

use ez_quick_xml::{quick_xml, MoreWriter, Reader};
use quick_xml::{Writer, de::from_str};

#[derive(Debug, Copy, Clone)]
pub struct Controls {
    pub escape:             KeyCode,
    pub up:                 KeyCode,
    pub down:               KeyCode,
    pub select:             KeyCode,
    pub right:              KeyCode,
    pub left:               KeyCode,
    pub toggle:             KeyCode,
    pub help:               KeyCode,
    pub save:               KeyCode,
    pub print:              KeyCode,
    pub print_unfinished:   KeyCode,
    pub delete:             KeyCode,
    pub copy:               KeyCode,
    pub move_entry:         KeyCode,
}

impl Controls {
    pub fn default() -> Controls {
        Controls {
            escape:             KeyCode::Esc,
            up:                 KeyCode::Up,
            down:               KeyCode::Down,
            select:             KeyCode::Enter,
            right:              KeyCode::Right,
            left:               KeyCode::Left,
            toggle:             KeyCode::Char(' '),
            help:               KeyCode::Char('h'),
            save:               KeyCode::Char('s'),
            print:              KeyCode::Char('p'),
            print_unfinished:   KeyCode::Char('o'),
            delete:             KeyCode::Delete,
            copy:               KeyCode::Char('c'),
            move_entry:         KeyCode::Char('m'),
        }
    }

    pub fn get_event(manager: &mut Manager) -> KeyCode {
        loop {
            match read().unwrap() {
                Event::Mouse(_) => (),
                Event::Resize(x, y) => {
                    manager.resize(x, y);
                    manager.draw().unwrap();
                },
                Event::Key(event) => {
                    return event.code
                }
            }
        }
    }

    pub fn help_tip(&self) -> String {
        format!("Press {} to open the help menu.", self.help.display_quot())
    }

    pub fn main(&self, manager: &mut Manager) -> MainControls {
        use MainControls::*;

        loop {
            match Controls::get_event(manager) {
                x if x == self.escape   => {return Esc}
                x if x == self.up       => {return PointerUp}
                x if x == self.down     => {return PointerDown}
                x if x == self.select   => {return Select}
                x if x == self.help     => {return Help}
                _ => (),
            }
        }
    }

    pub fn item(&self, manager: &mut Manager) -> ItemControls {
        use ItemControls::*;

        loop {
            match Controls::get_event(manager) {
                x if x == self.escape           => {return Esc}
                x if x == self.up               => {return PointerUp}
                x if x == self.down             => {return PointerDown}
                x if x == self.select           => {return Select}
                x if x == self.right            => {return IncDepth}
                x if x == self.left             => {return DecDepth}
                x if x == self.toggle           => {return Toggle}
                x if x == self.save             => {return Save}
                x if x == self.help             => {return Help}
                x if x == self.print            => {return Print}
                x if x == self.print_unfinished => {return PrintUnfinished}
                x if x == self.delete           => {return Delete}
                x if x == self.copy             => {return Copy}
                x if x == self.move_entry       => {return Move}
                _ => (),
            }
        }
    }

    pub fn item_prompt(&self) -> String {
        format!("Exit: \"{}\", Up: \"{}\", Down: \"{}\", Increase Depth: \"{}\", Decrease Depth: \"{}\", Select: \"{}\"\n{}", 
            self.escape.display(),
            self.up.display(),
            self.down.display(),
            self.right.display(),
            self.left.display(),
            self.select.display(),
            self.help_tip()
        )
    }

    pub fn help(&self, manager: &mut Manager) -> HelpControls {
        use HelpControls::*;

        loop {
            match Controls::get_event(manager) {
                x if x == self.escape => {return Esc}
                x if x == self.help   => {return Esc}
                x if x == self.up     => {return Up}
                x if x == self.down   => {return Down}
                _ => (),
            }
        }
    }

    pub fn help_prompt(&self) -> String {
        format!("Exit: \"{}\" or \"{}\", Scroll Up: \"{}\", Scroll Down: \"{}\"", 
            self.escape.display(),
            self.help.display(),
            self.up.display(),
            self.down.display(),
        )
    }

    pub fn multi_select(&self, manager: &mut Manager) -> Selection {
        use Selection::*;

        loop {
            match Controls::get_event(manager) {
                x if x == self.escape => {return Esc}
                x if x == self.up     => {return PointerUp}
                x if x == self.down   => {return PointerDown}
                x if x == self.select => {return Select}
                _ => (),
            }
        }
    }

    pub fn select_prompt(&self) -> String {
        format!("Cancel: \"{}\", Up: \"{}\", Down: \"{}\", Select: \"{}\"", 
            self.escape.display(),
            self.up.display(),
            self.down.display(),
            self.select.display(),
        )
    }

    pub fn position(&self, manager: &mut Manager) -> Position {
        use Position::*;

        loop {
            match Controls::get_event(manager) {
                x if x == self.up    => {return SiblingBefore}
                x if x == self.down  => {return SiblingAfter}
                x if x == self.left  => {return FirstChild}
                x if x == self.right => {return LastChild}
                _ => (),
            }
        }
    }

    pub fn position_prompt(&self) -> String {
        format!("Where? first child: \"{}\", last child: \"{}\", sibling above: \"{}\", Sibling below: \"{}\"", 
            self.left.display(),
            self.right.display(),
            self.up.display(),
            self.down.display(),
        )
    }

    pub fn position_root_prompt(&self) -> String {
        format!("Where? first child: \"{}\", last child: \"{}\"", 
            self.left.display(),
            self.right.display(),
        )
    }

    pub fn save<W: std::io::Write>(&self, writer: &mut Writer<W>) -> quick_xml::Result<()> {
        use quick_xml::events::*;

        let name = BytesStart::borrowed_name(b"controls");
        writer.write_event(Event::Start(name.to_borrowed()))?;

            writer.named_value(b"escape",           &self.escape)?;
            writer.named_value(b"up",               &self.up)?;
            writer.named_value(b"down",             &self.down)?;
            writer.named_value(b"select",           &self.select)?;
            writer.named_value(b"right",            &self.right)?;
            writer.named_value(b"left",             &self.left)?;
            writer.named_value(b"toggle",           &self.toggle)?;
            writer.named_value(b"help",             &self.help)?;
            writer.named_value(b"save",             &self.save)?;
            writer.named_value(b"print",            &self.print)?;
            writer.named_value(b"print_unfinished", &self.print_unfinished)?;
            writer.named_value(b"delete",           &self.delete)?;
            writer.named_value(b"copy",             &self.copy)?;
            writer.named_value(b"move_entry",       &self.move_entry)?;

        writer.write_event(Event::End(name.to_end()))?;
        Ok(())
    }

    pub fn load<B: std::io::BufRead>(&mut self, reader: &mut Reader<B>) -> quick_xml::Result<()> {
        try_load!(
            reader,
            {self.escape, b"escape"},
            {self.up, b"up"},
            {self.down, b"down"},
            {self.select, b"select"},
            {self.right, b"right"},
            {self.left, b"left"},
            {self.toggle, b"toggle"},
            {self.help, b"help"},
            {self.save, b"save"},
            {self.print, b"print"},
            {self.print_unfinished, b"print_unfinished"},
            {self.delete, b"delete"},
            {self.copy, b"copy"},
            {self.move_entry, b"move_entry"}
        );

        Ok(())
    }

    pub fn index_field(&mut self, index: usize) -> Option<&mut KeyCode> {
        match index {
            0  => Some(&mut self.escape),
            1  => Some(&mut self.up),
            2  => Some(&mut self.down),
            3  => Some(&mut self.select),
            4  => Some(&mut self.right),
            5  => Some(&mut self.left),
            6  => Some(&mut self.toggle),
            7  => Some(&mut self.help),
            8  => Some(&mut self.save),
            9  => Some(&mut self.print),
            10 => Some(&mut self.print_unfinished),
            11 => Some(&mut self.delete),
            12 => Some(&mut self.copy),
            13 => Some(&mut self.move_entry),
            _ => None
        }
    }
}