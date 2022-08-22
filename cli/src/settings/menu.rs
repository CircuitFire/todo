use frames::{Manager, crossterm::style::Color};
use super::*;
use crate::*;
use core::{Formatter, Entry, Format, Type};

use MainControls::*;

pub struct SettingMenu {
    main: ListUi,
    controls: ListUi,
    colors: ListUi,
    formatter: ListUi,
}

impl SettingMenu {
    pub fn new(manager: &mut Manager, colors: &Colors, data: &SettingData) -> Self {
        let mut main = ListUi::new(manager, Some(Box::new(MainUiUpdate{})), colors);
        main.push(String::from("Controls"));
        main.push(String::from("Colors"));
        main.push(String::from("Formatter"));
        main.push(String::from("Print Formatter"));
        main.push(String::from("Apply Changes"));

        main.set_pointer(0, manager, colors);
        main.disable();

        SettingMenu {
            main: main,
            controls: SettingMenu::new_control_menu(manager, colors, &data.controls),
            colors: SettingMenu::new_color_menu(manager, colors),
            formatter: SettingMenu::new_formatter_menu(manager, colors),
        }
    }

    pub fn main(
        &mut self,
        manager: &mut Manager,
        help:    &mut HelpMenu,
        prompt:  &mut Prompt,
        cur_data: &mut SettingData,
    ) -> bool {
        self.main.enable();
        self.main.update(manager);
        prompt.set_prompt(manager, cur_data.controls.help_tip());

        let mut changes = false;

        let mut temp_colors = cur_data.colors.clone();
        let mut temp_controls = cur_data.controls.clone();
        let mut temp_formatter = cur_data.formatter.clone();
        let mut temp_print_formatter = cur_data.print_formatter.clone();

        loop {
            manager.draw().unwrap();

            match cur_data.controls.main(manager){
                Esc         => break,
                PointerUp   => self.main.dec_pointer(manager, &cur_data.colors, 1),
                PointerDown => self.main.inc_pointer(manager, &cur_data.colors, 1),
                Help        => {
                    self.main.disable();
                    prompt.set_prompt(manager, cur_data.controls.help_prompt());
                    help.main(manager, &cur_data.controls, HelpMenuType::Setting);
                    self.main.enable();
                    self.main.update(manager);
                    prompt.set_prompt(manager, cur_data.controls.help_tip());
                },
                Select      => {
                    match self.main.pointer() {
                        0 => self.controls(manager, help, prompt, cur_data, &mut temp_controls),
                        1 => self.colors(manager, help, prompt, cur_data, &mut temp_colors),
                        2 => self.formatter(manager, help, prompt, cur_data, &mut temp_formatter),
                        3 => self.formatter(manager, help, prompt, cur_data, &mut temp_print_formatter),
                        4 => {
                            cur_data.colors = temp_colors.clone();
                            cur_data.controls = temp_controls.clone();
                            cur_data.formatter = temp_formatter.clone();
                            cur_data.print_formatter = temp_print_formatter.clone();

                            self.refresh_colors(&cur_data.colors, manager);
                            prompt.refresh_colors(&cur_data.colors, manager);
                            help.refresh_colors(&cur_data.colors, manager);

                            changes = true;
                        }
                        _ => ()
                    }
                    self.main.update(manager);
                },
            }
        }

        self.main.disable();

        changes
    }

    pub fn refresh_colors(&mut self, colors: &Colors, manager: &mut Manager) {
        self.main.refresh_colors(colors, manager);
        self.controls.refresh_colors(colors, manager);
        self.colors.refresh_colors(colors, manager);
        self.colors.set_fg(6, colors.pointer);
        self.colors.set_fg(7, colors.selected);
        self.formatter.refresh_colors(colors, manager);
    }

    fn new_control_menu(manager: &mut Manager, colors: &Colors, controls: &Controls) -> ListUi {
        let mut menu = ListUi::new(manager, Some(Box::new(MainUiUpdate{})), colors);
        menu.disable();

        menu.push(format!("Escape           : {}", controls.escape.display_quot()));
        menu.push(format!("Up               : {}", controls.up.display_quot()));
        menu.push(format!("Down             : {}", controls.down.display_quot()));
        menu.push(format!("Select           : {}", controls.select.display_quot()));
        menu.push(format!("Right            : {}", controls.right.display_quot()));
        menu.push(format!("Left             : {}", controls.left.display_quot()));
        menu.push(format!("Toggle           : {}", controls.toggle.display_quot()));
        menu.push(format!("Help             : {}", controls.help.display_quot()));
        menu.push(format!("Save             : {}", controls.save.display_quot()));
        menu.push(format!("Print            : {}", controls.print.display_quot()));
        menu.push(format!("Print Unfinished : {}", controls.print_unfinished.display_quot()));
        menu.push(format!("Delete           : {}", controls.delete.display_quot()));
        menu.push(format!("Copy             : {}", controls.copy.display_quot()));
        menu.push(format!("Move Entry       : {}", controls.move_entry.display_quot()));

        menu.set_pointer(0, manager, colors);
        menu
    }

    fn controls(&mut self,
        manager:  &mut Manager,
        help:     &mut HelpMenu,
        prompt:   &mut Prompt,
        cur_data: &mut SettingData,
        temp:     &mut Controls) {

        self.main.disable();
        self.controls.enable();
        self.controls.update(manager);

        loop {
            manager.draw().unwrap();

            match cur_data.controls.main(manager){
                Esc         => break,
                PointerUp   => self.controls.dec_pointer(manager, &cur_data.colors, 1),
                PointerDown => self.controls.inc_pointer(manager, &cur_data.colors, 1),
                Help        => {
                    self.controls.disable();
                    prompt.set_prompt(manager, cur_data.controls.help_prompt());
                    help.main(manager, &cur_data.controls, HelpMenuType::Setting);
                    self.controls.enable();
                    self.controls.update(manager);
                    prompt.set_prompt(manager, cur_data.controls.help_tip());
                },
                Select => self.controls_select(manager, prompt, cur_data, temp),
            }
        }

        self.controls.disable();
        self.main.enable();
    }

    fn controls_select(&mut self, manager: &mut Manager, prompt: &mut Prompt, cur_data: &mut SettingData, temp: &mut Controls) {
        prompt.set_prompt(manager, String::from("Press key to set new control"));

        let i = self.controls.pointer();

        let control = temp.index_field(i).unwrap();
        *control = Controls::get_event(manager);

        change_control(&mut self.controls, i, &control.display());

        self.controls.update(manager);
        prompt.set_prompt(manager, cur_data.controls.help_prompt());
    }

    fn new_color_menu(manager: &mut Manager, colors: &Colors) -> ListUi {
        let mut menu = ListUi::new(manager, Some(Box::new(MainUiUpdate{})), colors);
        menu.disable();

        menu.push(format!("default    : {}", colors.default.display()));
        menu.push(format!("background : {}", colors.background.display()));
        menu.push(format!("pointer    : {}", colors.pointer.display()));
        menu.push(format!("selected   : {}", colors.selected.display()));

        menu.push(String::from("\nExamples:"));
        menu.push(String::from("Default"));
        menu.push(String::from("Pointer"));
        menu.set_fg(6, colors.pointer);
        menu.push(String::from("Selected"));
        menu.set_fg(7, colors.selected);

        menu.set_pointer(0, manager, colors);
        menu
    }

    fn colors(&mut self,
        manager: &mut Manager,
        help:    &mut HelpMenu,
        prompt:  &mut Prompt,
        cur_data: &mut SettingData,
        temp:     &mut Colors) {
        
        self.main.disable();
        self.colors.enable();
        self.colors.update(manager);

        loop {
            manager.draw().unwrap();

            match cur_data.controls.main(manager){
                Esc         => break,
                PointerUp   => {
                    if self.colors.pointer() == 0 {
                        self.colors.set_pointer(3, manager, &cur_data.colors);
                    }
                    else {
                        self.colors.dec_pointer(manager, &cur_data.colors, 1)
                    }
                }
                PointerDown => {
                    if self.colors.pointer() == 3 {
                        self.colors.set_pointer(0, manager, &cur_data.colors);
                    }
                    else {
                        self.colors.inc_pointer(manager, &cur_data.colors, 1)
                    }
                }
                Help        => {
                    self.colors.disable();
                    prompt.set_prompt(manager, cur_data.controls.help_prompt());
                    help.main(manager, &cur_data.controls, HelpMenuType::Setting);
                    self.colors.enable();
                    self.colors.update(manager);
                    prompt.set_prompt(manager, cur_data.controls.help_tip());
                },
                Select => self.color_select(manager, prompt, cur_data, temp),
            }
        }

        self.colors.disable();
        self.main.enable();
    }

    fn color_select(&mut self, manager: &mut Manager, prompt: &mut Prompt, cur_data: &mut SettingData, temp: &mut Colors) {
        let red = loop {
            if let Ok(new) = prompt.get_string(manager, String::from("Enter new red value ( 0 - 255 )")).parse() {
                break new
            }
        };
        let green = loop {
            if let Ok(new) = prompt.get_string(manager, String::from("Enter new green value ( 0 - 255 )")).parse() {
                break new
            }
        };
        let blue = loop {
            if let Ok(new) = prompt.get_string(manager, String::from("Enter new blue value ( 0 - 255 )")).parse() {
                break new
            }
        };

        let i = self.colors.pointer();

        let color = temp.index_field(i).unwrap();
        *color = Color::Rgb{r: red, g: green, b: blue};
        
        change_control(&mut self.colors, i, &color.display());
        self.colors_example(temp);

        self.colors.update(manager);
        prompt.set_prompt(manager, cur_data.controls.help_prompt());
    }

    fn colors_example(&mut self, colors: &Colors) {
        self.colors.set_fg(5, colors.default);
        self.colors.set_bg(5, colors.background);
        self.colors.set_fg(6, colors.pointer);
        self.colors.set_bg(6, colors.background);
        self.colors.set_fg(7, colors.selected);
        self.colors.set_bg(7, colors.background);
    }

    fn new_formatter_menu(manager: &mut Manager, colors: &Colors) -> ListUi {
        let mut menu = ListUi::new(manager, Some(Box::new(MainUiUpdate{})), colors);
        menu.disable();

        menu.push(String::from("Indent     :"));
        menu.push(String::from("Completed  :"));
        menu.push(String::from("Incomplete :"));
        menu.push(String::from("Type       :"));

        menu.push(String::from("\nExample:"));
        menu.push(String::new());
        menu.push(String::new());
        menu.push(String::new());
        menu.push(String::new());

        menu.set_pointer(0, manager, colors);
        menu
    }

    fn formatter_example(&mut self, formatter: &Formatter) {
        self.formatter.set_text(5, formatter.format(&Entry{name: String::from("Root"),          complete: false}, 0, true));
        self.formatter.set_text(6, formatter.format(&Entry{name: String::from("Child 1"),       complete: true},  1, true));
        self.formatter.set_text(7, formatter.format(&Entry{name: String::from("Child 1 child"), complete: true},  2, false));
        self.formatter.set_text(8, formatter.format(&Entry{name: String::from("Child 2"),       complete: false}, 1, false));
    }

    fn fill_formatter_menu(&mut self, temp: &mut Formatter) {
        change_control(&mut self.formatter, 0, &format!("{}", temp.get_indent()));
        change_control(&mut self.formatter, 1, &format!("\"{}\"", temp.get_completed()));
        change_control(&mut self.formatter, 2, &format!("\"{}\"", temp.get_incomplete()));
        change_control(&mut self.formatter, 3, &temp.get_type().display_quot());
    }

    fn formatter(&mut self,
        manager: &mut Manager,
        help:    &mut HelpMenu,
        prompt:  &mut Prompt,
        cur_data: &mut SettingData,
        temp:     &mut Formatter) {

        self.main.disable();
        self.formatter.enable();
        self.formatter.update(manager);

        self.fill_formatter_menu(temp);
        self.formatter_example(temp);

        loop {
            manager.draw().unwrap();

            match cur_data.controls.main(manager){
                Esc         => break,
                PointerUp   => {
                    if self.formatter.pointer() == 0 {
                        self.formatter.set_pointer(3, manager, &cur_data.colors);
                    }
                    else {
                        self.formatter.dec_pointer(manager, &cur_data.colors, 1)
                    }
                }
                PointerDown => {
                    if self.formatter.pointer() == 3 {
                        self.formatter.set_pointer(0, manager, &cur_data.colors);
                    }
                    else {
                        self.formatter.inc_pointer(manager, &cur_data.colors, 1)
                    }
                }
                Help        => {
                    self.formatter.disable();
                    prompt.set_prompt(manager, cur_data.controls.help_prompt());
                    help.main(manager, &cur_data.controls, HelpMenuType::Setting);
                    self.formatter.enable();
                    self.formatter.update(manager);
                    prompt.set_prompt(manager, cur_data.controls.help_tip());
                },
                Select => self.formatter_select(manager, prompt, cur_data, temp),
            }
        }

        self.formatter.disable();
        self.main.enable();
    }

    fn formatter_select(&mut self, manager: &mut Manager, prompt: &mut Prompt, cur_data: &mut SettingData, temp: &mut Formatter) {
        match self.formatter.pointer() {
            0 => {
                if let Ok(num) = prompt.get_string(manager, String::from("Enter how deep the indent should be")).parse() {
                    temp.set(Some(num), None, None, None);
                    change_control(&mut self.formatter, 0, &format!("{}", temp.get_indent()));
                    self.formatter_example(temp);
                }
            }
            1 => {
                if let Some(new) = prompt.get_string_no_trim(manager, String::from("Enter a new character to use when a task is complete")).chars().next() {
                    temp.set(None, Some(new), None, None);
                    change_control(&mut self.formatter, 1, &format!("\"{}\"", temp.get_completed()));
                    self.formatter_example(temp);
                }
            }
            2 => {
                if let Some(new) = prompt.get_string_no_trim(manager, String::from("Enter a new character to use when a task is incomplete complete")).chars().next() {
                    temp.set(None, None, Some(new), None);
                    change_control(&mut self.formatter, 2, &format!("\"{}\"", temp.get_incomplete()));
                    self.formatter_example(temp);
                }
            }
            3 => {
                let new = match temp.get_type() {
                    Type::Basic => Type::Fancy,
                    Type::Fancy => Type::Basic,
                };
                temp.set(None, None, None, Some(new));
                change_control(&mut self.formatter, 3, &temp.get_type().display_quot());
                self.formatter_example(temp);
            }
            _ => ()
        }

        self.formatter.update(manager);
        prompt.set_prompt(manager, cur_data.controls.help_prompt());
    }
}

fn change_control(menu: &mut ListUi, index: usize, new: &str) {
    let mut temp = {
        let borrow = menu.frame().borrow();
        let (temp, _) = borrow.get_text(index).split_once(':').unwrap();
        String::from(temp)
    };

    temp.push_str(": ");
    temp.push_str(new);
    
    menu.set_text(index, temp);
}