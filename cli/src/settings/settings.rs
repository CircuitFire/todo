use frames::{Manager, Pixel};
use crate::*;
use super::*;
use core::Formatter;
use menu::SettingMenu;

use std::{io, fs, path::{PathBuf, Path}};
use ez_quick_xml::*;
use quick_xml::{Writer, events::*, de::from_str};

pub struct SettingData {
    pub controls: Controls,
    pub colors:   Colors,
    pub formatter:       Formatter,
    pub print_formatter: Formatter,
}

///Contains settings that are used throughout the program.
pub struct Settings{
    menu: SettingMenu,
    help: HelpMenu,

    data: SettingData,

    conf_dir: Option<PathBuf>
}

impl Settings {
    ///Creates settings object and a new frame manager.
    pub fn new() -> quick_xml::Result<(Settings, Manager)> {
        let colors = Colors::new();

        let mut manager = Manager::new_fill(Pixel::new(' ', colors.default, colors.background))?;
        let data = SettingData {
            controls: Controls::default(),
            colors: colors,
            formatter:       Formatter::default(),
            print_formatter: Formatter::default(),
        };

        let mut settings = Settings {
            menu: SettingMenu::new(&mut manager, &colors, &data),
            help: HelpMenu::new(&mut manager, &colors),
            data: data,
            conf_dir: Settings::gen_path(),
        };

        if let Err(_) = settings.load() {}

        settings.menu.refresh_colors(&settings.data.colors, &mut manager);
        settings.help.refresh_colors(&settings.data.colors, &mut manager);

        Ok((
            settings,
            manager
        ))
    }

    pub fn reset() {
        if let Some(path) = Settings::gen_path() {
            let mut path = PathBuf::from(path);
            path.push("config.xml");

            if let Ok(_) = fs::remove_file(path) {}
        }
    }

    pub fn main(&mut self, manager: &mut Manager, prompt: &mut Prompt) -> bool {

        let changes = self.menu.main(
            manager,
            &mut self.help,
            prompt,
            &mut self.data,
        );

        if changes {
            if let Err(x) = self.save() {
                //todo some indication that saving has failed.
            }
        }

        changes
    }

    ///Returns a reference of the control object.
    pub fn controls(&self) -> &Controls {
        &self.data.controls
    }

    pub fn colors(&self) -> &Colors {
        &self.data.colors
    }

    ///Returns the formatter used for the drawing the list to the screen.
    pub fn formatter(&self) -> &Formatter{
        &self.data.formatter
    }

    ///Returns the formatter that is used for saving the list to a txt file.
    pub fn print_formatter(&self) -> &Formatter{
        &self.data.print_formatter
    }

    ///Switches to the help menu for the given type.
    pub fn help_menu(&mut self, manager: &mut Manager, prompt: &mut Prompt, help_type: HelpMenuType) {
        prompt.set_prompt(manager, self.data.controls.help_prompt());
        self.help.main(manager, &self.data.controls, help_type);
    }

    ///Saves the the current settings into a file.
    pub fn save(&mut self) -> quick_xml::Result<()> {
        if let Some(path) = self.conf_file("config.xml"){
            fs::create_dir_all(path.parent().unwrap())?;

            let file = fs::OpenOptions::new().write(true).create(true).open(&path)?;
            let file = io::BufWriter::new(file);
            
            let mut writer = Writer::new_with_indent(file, b'\t', 1);

            writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
            writer.write_indent()?;

            //writer.write_event()?;
            let settings = BytesStart::borrowed_name(b"settings");
            writer.write_event(Event::Start(settings.to_borrowed()))?;

                self.data.colors.save(&mut writer)?;
                self.data.controls.save(&mut writer)?;
                Settings::save_formatter(&mut writer, &self.data.formatter, b"formatter")?;
                Settings::save_formatter(&mut writer, &self.data.print_formatter, b"print_formatter")?;

            writer.write_event(Event::End(settings.to_end()))?;
        }

        Ok(())
    }

    fn save_formatter<W: io::Write>(writer: &mut Writer<W>, formatter: &Formatter, name: &[u8]) -> quick_xml::Result<()> {
        let name = BytesStart::borrowed_name(name);
        writer.write_event(Event::Start(name.to_borrowed()))?;

            writer.named_value(b"completed",  &formatter.get_completed())?;
            writer.named_value(b"incomplete", &formatter.get_incomplete())?;
            writer.named_value(b"indent",     &formatter.get_indent())?;
            writer.named_value(b"type",       &formatter.get_type())?;

        writer.write_event(Event::End(name.to_end()))?;
        Ok(())
    }

    pub fn load(&mut self) -> quick_xml::Result<()> {
        if let Some(path) = self.conf_file("config.xml"){
            let mut reader = Reader::new(quick_xml::Reader::from_file(&path)?);

            if reader.find(b"settings")?.is_some() {
                loop {
                    match reader.next()? {
                        Some(e) if e.name() == b"colors" =>          self.data.colors.load(&mut reader)?,
                        Some(e) if e.name() == b"controls" =>        self.data.controls.load(&mut reader)?,
                        Some(e) if e.name() == b"formatter" =>       Settings::load_formatter(&mut self.data.formatter, &mut reader)?,
                        Some(e) if e.name() == b"print_formatter" => Settings::load_formatter(&mut self.data.print_formatter, &mut reader)?,
                        Some(_) => (),
                        None => break,
                    }
                }
            }
        }

        Ok(())
    }

    fn load_formatter<B: io::BufRead>(formatter: &mut Formatter, reader: &mut Reader<B>) -> quick_xml::Result<()> {
        let mut indent = None;
        let mut completed = None;
        let mut incomplete = None;
        let mut form_type = None;

        try_load_formatter!(
            reader,
            {indent, b"indent"},
            {completed, b"completed"},
            {incomplete, b"incomplete"},
            {form_type, b"form_type"}
        );

        formatter.set(indent, completed, incomplete, form_type);

        Ok(())
    }
/*
    fn load_formatter<B: io::BufRead>(formatter: &mut Formatter, reader: &mut Reader<B>) -> quick_xml::Result<()> {
        let mut indent = None;
        let mut completed = None;
        let mut incomplete = None;
        let mut form_type = None;

        loop {
            match reader.next()? {
                Some(e) if e.name() == b"indent" => {
                    if let Ok(temp) = reader.read_until(b"indent") {
                        indent = Some(from_str(&temp).unwrap());
                    }
                }
                Some(e) if e.name() == b"completed" => {
                    if let Ok(temp) = reader.read_until(b"completed") {
                        completed = Some(from_str(&temp).unwrap());
                    }
                }
                Some(e) if e.name() == b"incomplete" => {
                    if let Ok(temp) = reader.read_until(b"incomplete") {
                        incomplete = Some(from_str(&temp).unwrap());
                    }
                }
                Some(e) if e.name() == b"form_type" => {
                    if let Ok(temp) = reader.read_until(b"form_type") {
                        form_type = Some(from_str(&temp).unwrap());
                    }
                }
                Some(_) => (),
                None => break,
            }
        }

        formatter.set(indent, completed, incomplete, form_type);

        Ok(())
    }
*/
    pub fn conf_path(&self) -> Option<PathBuf> {
        if let Some(ref dir) = self.conf_dir {
            Some(PathBuf::from(dir))
        }
        else {
            None
        }
    }

    pub fn conf_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        if let Some(ref dir) = self.conf_dir {
            let mut dir = PathBuf::from(dir);
            dir.push(path);
            Some(dir)
        }
        else {
            None
        }
    }

    ///Gen config file path. only works with linux and windows.
    #[cfg(target_os = "linux")]
    fn gen_path() -> Option<PathBuf>{
        if let Ok(path) = std::env::var("HOME") {
            let mut path = PathBuf::from(path);
            path.push(".config/CircuitFire/todo/");
            Some(path)
        }
        else{
            None
        }
    }

    ///Gen config file path. only works with linux and windows.
    #[cfg(target_os = "windows")]
    fn gen_path() -> Option<PathBuf>
    {
        if let Ok(path) = std::env::var("appdata") {
            let mut path = PathBuf::from(path);
            path.push("CircuitFire/todo");
            Some(path)
        }
        else{
            None
        }
    }
}

macro_rules! try_load {
    ($reader:ident, $({$var:expr, $name:literal}),*) => {
        loop {
            match $reader.next()? {
                $(Some(e) if e.name() == $name => {
                    if let Ok(temp) = $reader.read_until_str($name) {
                        if let Ok(temp) = from_str(&temp) {
                            $var = temp;
                        }
                    }
                })*
                Some(_) => (),
                None => break,
            }
        }
    };
}

pub(crate) use try_load;

macro_rules! try_load_formatter {
    ($reader:ident, $({$var:expr, $name:literal}),*) => {
        loop {
            match $reader.next()? {
                $(Some(e) if e.name() == $name => {
                    if let Ok(temp) = $reader.read_until_str($name) {
                        if let Ok(temp) = from_str(&temp) {
                            $var = Some(temp);
                        }
                    }
                })+
                Some(_) => (),
                None => break,
            }
        }
    };
}

pub(crate) use try_load_formatter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (mut test, _) = Settings::new().unwrap();

        println!("{:?}", test.conf_file("config.xml"));

        test.save().unwrap();
    }

    #[test]
    fn test_2() {
        let xml = "<default><Rgb r=\"255\" g=\"255\" b=\"255\"/></default>";

        let mut reader = quick_xml::Reader::from_str(xml);

        let mut buf = Vec::new();

        //let mut found = None;

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    println!("{}", std::str::from_utf8(e.name()).unwrap());
                    //found = Some(e.to_owned());
                    buf.clear();
                    break;
                },
                Ok(Event::Eof) => break,
                Ok(x) => {
                    println!("{:?}", x);
                }
                _ => {println!("err");},
            }
        }

        //if let Some(found) = found {
        //    let temp = Settings::load_contents(&mut reader, found.name(), &mut buf).unwrap();
        //    println!("buffer: {}", temp);
        //}
    }
}