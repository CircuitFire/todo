use std::io::stdout;
use std::path::PathBuf;

use super::*;
use frames::*;

use crossterm::{terminal, ExecutableCommand};
use terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ez_quick_xml::quick_xml;

pub enum MainControls {
    Esc,
    PointerUp,
    PointerDown,
    Select,
    Help,
}

pub struct Todo {
    manager: Manager,
    menu: ListUi,
    list: ItemList,
    prompt: Prompt,
    settings: Settings,
    files: Vec<PathBuf>,
}

enum LeaveTo {
    ItemList,
    ManLoadList,
    LoadList,
    Help,
    Settings,
}

impl Todo{
    pub fn new(reset: bool) -> quick_xml::Result<Todo> {
        if reset {
            Settings::reset();
        }

        let (settings, mut manager) = Settings::new()?;

        manager.add_task(Task::UpdateAll);
        let list = ItemList::new(&mut manager, &settings.colors());

        let mut menu = ListUi::new(&mut manager, Some(Box::new(MainUiUpdate{})), &settings.colors());

        menu.push(String::from("New Todo List."));
        menu.push(String::from("Load List."));
        menu.push(String::from("Settings."));

        Ok(Todo{
            list: list,
            menu: menu,
            prompt: Prompt::new(&mut manager, &settings.colors()),
            manager: manager,
            settings: settings,
            files: Vec::new(),
        })
    }

    pub fn main(&mut self){
        use MainControls::*;

        stdout().execute(terminal::SetTitle("todo")).unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

        self.manager.match_size().unwrap();
        self.write_menu();
        self.menu.set_pointer(0, &mut self.manager, &self.settings.colors());
        self.set_prompt_selected();
        
        loop {
            self.set_prompt_selected();
            self.manager.draw().unwrap();

            match self.settings.controls().main(&mut self.manager){
                Esc         => break,
                PointerUp   => self.pointer_up(),
                PointerDown => self.pointer_down(),
                Select      => self.select(),
                Help        => self.leave_to(LeaveTo::Help),
            }
        }

        terminal::disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    fn pointer_up(&mut self) {
        self.menu.dec_pointer(&mut self.manager, &self.settings.colors(), 1);
    }

    fn pointer_down(&mut self) {
        self.menu.inc_pointer(&mut self.manager, &self.settings.colors(), 1);
    }

    fn leave_to(&mut self, place: LeaveTo) {
        use LeaveTo::*;

        self.menu.disable();

        match place {
            ItemList => {
                let mut data = {
                    let name = self.get_string(String::from("Name of new list."));
                    self.list.new_list(name)
                };
                
                self.list.main(&mut data, &mut self.settings, &mut self.manager, &mut self.prompt);
            },
            ManLoadList => {
                let data = {
                    let file = PathBuf::from(self.get_string(String::from("List location.")));
                    self.list.load_list(&file)
                };
                
                if let Ok(mut data) = data {
                    self.list.main(&mut data, &mut self.settings, &mut self.manager, &mut self.prompt);
                }
            },
            LoadList => {
                let data = self.load_selected_file();

                if let Ok(mut data) = data {
                    self.list.main(&mut data, &mut self.settings, &mut self.manager, &mut self.prompt);
                }
            },
            Help => {
                self.settings.help_menu(&mut self.manager, &mut self.prompt, HelpMenuType::Main);
            },
            Settings => {
                if self.settings.main(&mut self.manager, &mut self.prompt) {
                    self.menu.refresh_colors(&self.settings.colors(), &mut self.manager)
                }
            }
        }

        self.menu.enable();
        self.menu.update(&mut self.manager);
    }

    fn select(&mut self){

        match self.menu.pointer() {
            0 => self.leave_to(LeaveTo::ItemList),
            1 => self.leave_to(LeaveTo::ManLoadList),
            2 => self.leave_to(LeaveTo::Settings),
            3 => self.set_dir(),
            _ => self.leave_to(LeaveTo::LoadList),
        }
    }

    fn get_string(&mut self, text: String) -> String {
        self.prompt.get_string(&mut self.manager, text)
    }

    fn set_prompt_selected(&mut self) {
        match self.menu.pointer() {
            0 => self.set_prompt("Select to create a new todo list."),
            1 => self.set_prompt("Select to load a todo list."),
            2 => self.set_prompt("Select to change settings."),
            3 => self.set_prompt("Select to change the current directory."),
            _ => self.set_prompt("Select to open file."),
        }
    }

    fn set_prompt(&mut self, text: &str) {
        self.prompt.set_prompt(&mut self.manager, format!(
            "{}\n{}",
            text,
            &self.settings.controls().help_tip()
        ));
    }

    fn write_menu(&mut self) {
        self.menu.truncate(3);
        self.files.clear();

        let dir = std::env::current_dir().unwrap();
        self.menu.push(format!("Current Dir: {}", dir.display()));

        for file in std::fs::read_dir(dir).unwrap(){
            if let Ok(file) = file{
                let file = file.path();
                if file.as_path().extension() == Some(std::ffi::OsStr::new("todo")) {
                    self.menu.push(String::from(file.file_stem().unwrap().to_str().unwrap()));
                    self.files.push(file);
                }
            }
        }

        self.menu.color_pointer(&self.settings.colors());
        self.menu.update(&mut self.manager);
    }

    fn set_dir(&mut self) {
        let input = self.prompt.get_string(&mut self.manager, String::from("Enter new Dir."));

        if std::env::set_current_dir(PathBuf::from(input)).is_ok() {
            self.write_menu();
        }
    }

    fn load_selected_file(&mut self) -> Result<Data, bytebuffer::ByteErr> {
        self.list.load_list(&self.files[self.menu.pointer() - 4])
    }
}