use frames::*;
use crossterm::event::{read, Event};

use super::Ui;
use super::listui::ListUi;
use super::prompt::*;
use super::ControlSettings;
use super::Control;


pub struct ConfigMan{
    config_ui: ListUi,
}

impl ConfigMan{
    pub fn new(ui: &Ui, controls: &ControlSettings) -> ConfigMan {
        let mut temp = ListUi::new(ui.get_config_obj(), ui.get_config_frame());

        temp.add_entry(&ui.color_string(0));
        temp.add_entry(&ui.color_string(1));

        for i in 0..controls.len(){
            temp.add_entry(&controls.to_string(i));
        }

        temp.color_pointer(0, &ui.colors);
        ConfigMan{
            config_ui: temp,
        }
    }

    pub fn inc_pointer(&mut self, ui: &mut Ui, inc: usize){
        self.config_ui.inc_pointer(ui, inc);
        ui.update_config();
        ui.draw();
    }

    pub fn dec_pointer(&mut self, ui: &mut Ui, dec: usize){
        self.config_ui.dec_pointer(ui, dec);
        ui.update_config();
        ui.draw();
    }

    pub fn set(&mut self, ui: &mut Ui, controls: &mut ControlSettings) -> bool {
        let ret = if self.config_ui.pointer() < 2 {
            ui.set_prompt(Prompt::SetR);
            ui.draw();
            let r = ui.read_user_u8();
            ui.set_prompt(Prompt::SetG);
            ui.draw();
            let g = ui.read_user_u8();
            ui.set_prompt(Prompt::SetB);
            ui.draw();
            let b = ui.read_user_u8();
            ui.colors[self.color_index()] = Color::Rgb{r: r, g: g, b: b};
            self.config_ui.set_entry_text(self.config_ui.pointer(), &ui.color_string(self.color_index()));
            self.config_ui.refresh_colors(&ui.colors);
            ui.refresh_colors();
            true
        }
        else{
            ui.set_prompt(Prompt::SetControl);
            ui.draw();
            loop{
                match read().unwrap() {
                    Event::Mouse(_) => (),
                    Event::Resize(width, height) => ui.resize(&Coord{x: width as i32, y: height as i32}),
                    Event::Key(event) => {
                        let index = self.control_index();
                        controls.set_key_at_num(index, event.code);
                        self.config_ui.set_entry_text(self.config_ui.pointer(), &controls.to_string(index));
                        ui.update_prompts(controls);
                        break;
                    },
                }
            }
            false
        };

        ui.update_config();
        ui.set_prompt(Prompt::ConfigControls);
        ui.draw();

        ret
    }

    pub fn set_default(&mut self, ui: &mut Ui, controls: &mut ControlSettings) -> bool{
        let ret = if self.config_ui.pointer() < 2 {
            ui.default_color(self.color_index());
            self.config_ui.set_entry_text(self.config_ui.pointer(), &ui.color_string(self.color_index()));
            self.config_ui.refresh_colors(&ui.colors);
            ui.refresh_colors();
            true
        }
        else{
            let index = self.control_index();
            controls.set_key_at_num(index, Control::from_num(index).default());
            self.config_ui.set_entry_text(self.config_ui.pointer(), &controls.to_string(index));
            ui.update_prompts(controls);
            false
        };

        ui.update_config();
        ui.set_prompt(Prompt::ConfigControls);
        ui.draw();

        ret
    }

    fn color_index(&self) -> usize{
        self.config_ui.pointer()
    }

    fn control_index(&self) -> usize{
        self.config_ui.pointer() - 2
    }
}