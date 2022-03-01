use frames::Color;

use std::io;
use ez_quick_xml::{quick_xml, Reader, MoreWriter};
use quick_xml::{Writer, events::*, de::from_str};

use crate::try_load;

#[derive(Copy, Clone)]
pub struct Colors{
    pub default:    Color,
    pub background: Color,
    pub pointer:    Color,
    pub selected:   Color,
}

impl Colors{
    pub fn new() -> Colors {
        Colors{
            default:    Color::Rgb{r: 255, g: 255, b: 255},
            background: Color::Rgb{r:   0, g:   0, b:   0},
            pointer:    Color::Rgb{r:   0, g: 255, b: 255},
            selected:   Color::Rgb{r: 255, g:   0, b: 255},
        }
    }

    pub fn save<W: io::Write>(&self, writer: &mut Writer<W>) -> quick_xml::Result<()> {
        let name = BytesStart::borrowed_name(b"colors");
        writer.write_event(Event::Start(name.to_borrowed()))?;

            writer.named_value(b"default",    &self.default)?;
            writer.named_value(b"background", &self.background)?;
            writer.named_value(b"pointer",    &self.pointer)?;
            writer.named_value(b"selected",   &self.selected)?;

        writer.write_event(Event::End(name.to_end()))?;
        Ok(())
    }

    pub fn load<B: io::BufRead>(&mut self, reader: &mut Reader<B>) -> quick_xml::Result<()> {
        try_load!(
            reader,
            {self.default, b"default"},
            {self.background, b"background"},
            {self.pointer, b"pointer"},
            {self.selected, b"selected"}
        );

        Ok(())
    }

    pub fn index_field(&mut self, index: usize) -> Option<&mut Color> {
        match index {
            0  => Some(&mut self.default),
            1  => Some(&mut self.background),
            2  => Some(&mut self.pointer),
            3  => Some(&mut self.selected),
            _ => None
        }
    }
}