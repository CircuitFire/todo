use frames::*;
use crossterm::event::{read, Event};

mod entry;
use entry::Entry;

mod prompt;
use prompt::*;

mod buffer;

mod ui;
use ui::*;

mod control;
use control::*;

mod entrymanager;
use entrymanager::EntryManager;

mod listui;

mod configman;
use configman::*;

mod display;

pub struct App{
    ui: Ui,
    control: ControlSettings,
    entries: EntryManager,
    config: ConfigMan,
}

impl App{
    pub fn new() -> Result<App, std::io::Error> {
        let controls = ControlSettings::new()?;

        let ui = Ui::new(&controls)?;

        Ok(App{
            entries: EntryManager::new(ui.get_entry_obj(), ui.get_entry_frame()),
            config: ConfigMan::new(&ui, &controls),
            ui: ui,
            control: controls,
        })
    }

    pub fn main(&mut self){
        if let Err(_) = std::fs::create_dir("./Lists"){}
        if self.entries.load_list(&mut self.ui).unwrap() {
            self.entries.new_list(&mut self.ui);
        }
        loop{
            match read().unwrap() {
                Event::Mouse(_) => (),
                Event::Resize(width, height) => self.ui.resize(&Coord{x: width as i32, y: height as i32}),
                Event::Key(event) => match self.control.keycode_to_control(event.code) {
                    Control::Up => self.entries.dec_pointer(&mut self.ui, 1),
                    Control::Down => self.entries.inc_pointer(&mut self.ui, 1),
                    Control::Left => self.entries.dec_depth(&mut self.ui, 1),
                    Control::Right => self.entries.inc_depth(&mut self.ui, 1),
                    Control::Select => self.entries.new_task(&mut self.ui),
                    Control::Esc => {
                        self.entries.save();
                        self.control.save().unwrap();
                        self.ui.save_colors().unwrap();
                        break;
                    },
                    Control::Del => {
                        if self.entries.delete_entry(&mut self.ui){break;}
                    },
                    Control::Complete => self.entries.toggle_complete(&mut self.ui),
                    Control::PrintAll => self.entries.print_formatted().unwrap(),
                    Control::PrintUnfinished => self.entries.print_unfinished().unwrap(),
                    Control::SetRoot => self.entries.change_root(&mut self.ui),
                    Control::BackRoot => self.entries.prev_root(&mut self.ui),
                    Control::Save => {
                        self.entries.save();
                        self.control.save().unwrap();
                        self.ui.save_colors().unwrap();
                    },
                    Control::Edit => self.entries.edit(&mut self.ui),
                    Control::Config => self.config(),
                    _ => (),
                },
            }
        }
    }

    fn config(&mut self){
        let mut color_update = false;

        self.ui.set_mode(Mode::Config);
        self.ui.set_prompt(Prompt::ConfigControls);
        self.ui.draw();
        loop{
            match read().unwrap() {
                Event::Mouse(_) => (),
                Event::Resize(width, height) => self.ui.resize(&Coord{x: width as i32, y: height as i32}),
                Event::Key(event) => match self.control.keycode_to_control(event.code) {
                    Control::Up => self.config.dec_pointer(&mut self.ui, 1),
                    Control::Down => self.config.inc_pointer(&mut self.ui, 1),
                    Control::Select => if self.config.set(&mut self.ui, &mut self.control){color_update = true},
                    Control::Esc => break,
                    Control::Del => if self.config.set_default(&mut self.ui, &mut self.control){color_update = true},
                    Control::Config => break,
                    _ => (),
                },
            }
        }
        if color_update {
            self.entries.refresh_colors(&self.ui.colors);
        }
        self.ui.set_mode(Mode::Entry);
        self.ui.set_prompt(Prompt::EntryControls);
        self.ui.draw();
    }
}