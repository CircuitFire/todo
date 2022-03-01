
use frames::*;
use crate::*;
use super::*;

#[derive(PartialEq, Debug)]
pub enum HelpMenuType {
    Main,
    Item,
    Setting,
}
use HelpMenuType::*;

pub enum HelpControls {
    Esc,
    Up,
    Down,
}

pub struct HelpMenu {
    object: Rc<RefCell<Object>>,
    frame: Rc<RefCell<frame_types::Text>>,
    pub current: HelpMenuType,
}

impl HelpMenu {
    pub fn new(manager: &mut Manager, colors: &Colors) -> HelpMenu {
        let frame = frame_types::Text::new(PixelData::new(' ', colors.default, colors.background));
        let obj = Object::new_min(frame.clone());

        {
            let mut borrowed = obj.borrow_mut();
            borrowed.size_update = Some(Box::new(MainUiUpdate{}));
            borrowed.enabled = false;
        }
        
        manager.objects.push(obj.clone());

        HelpMenu {
            object: obj,
            frame: frame,
            current: Main,
        }
    }

    pub fn refresh_colors(&mut self, colors: &Colors, manager: &mut Manager){
        let mut frame = self.frame.borrow_mut();
        frame.fill = PixelData::new(' ', colors.default, colors.background);
        if frame.len() == 0 {return}


        for i in 0..frame.len(){
            frame.set_fg(i, colors.default);
            frame.set_bg(i, colors.background);
        }

        manager.add_task(self.object.borrow().update());
    }

    pub fn main(&mut self, manager: &mut Manager, controls: &Controls, help_type: HelpMenuType) {
        use HelpControls::*;

        self.enable();

        self.set_menu(controls, help_type);
        manager.add_task(self.object.borrow().update());
        
        loop {
            manager.draw().unwrap();

            match controls.help(manager) {
                Esc  => break,
                Up   => self.up(manager),
                Down => self.down(manager),
            }
        }

        self.disable();
    }

    pub fn enable(&mut self) {
        self.object.borrow_mut().enabled = true;
    }

    pub fn disable(&mut self) {
        self.object.borrow_mut().enabled = false;
    }

    fn up(&mut self, manager: &mut Manager) {
        let mut object = self.object.borrow_mut();

        if object.offset.y != 0 {
            object.offset.y -= 1;
        }

        manager.add_task(object.update());
    }

    fn down(&mut self, manager: &mut Manager) {
        let mut object = self.object.borrow_mut();

        if object.offset.y < (self.frame.borrow().len() - 1) as i32 {
            object.offset.y += 1;
        }

        manager.add_task(object.update());
    }

    fn set_menu(&mut self, controls: &Controls, help_type: HelpMenuType) {
        if self.current == help_type {return}

        self.frame.borrow_mut().clear();
        self.object.borrow_mut().offset.y = 0;

        match help_type {
            Main => self.main_menu(controls),
            Item => self.item_menu(controls),
            Setting => self.settings_menu(controls),
        }
    }

    fn main_menu(&mut self, controls: &Controls){
        let mut frame = self.frame.borrow_mut();

        frame.push(format!("Exit         : {}", controls.escape.display_quot()));
        frame.push(format!("Pointer Up   : {}", controls.up.display_quot()));
        frame.push(format!("Pointer Down : {}", controls.down.display_quot()));
        frame.push(format!("Select       : {}", controls.select.display_quot()));
    }

    fn item_menu(&mut self, controls: &Controls){
        let mut frame = self.frame.borrow_mut();

        frame.push(format!("Exit                       : {}", controls.escape.display_quot()));
        frame.push(format!("Pointer Up                 : {}", controls.up.display_quot()));
        frame.push(format!("Pointer Down               : {}", controls.down.display_quot()));
        frame.push(format!("Increase Depth             : {}", controls.right.display_quot()));
        frame.push(format!("Decrease Depth             : {}", controls.left.display_quot()));
        frame.push(format!("Select                     : {}", controls.select.display_quot()));
        frame.push(format!("Save                       : {}", controls.save.display_quot()));
        frame.push(format!("Print into .txt            : {}", controls.print.display_quot()));
        frame.push(format!("Print unfinished into .txt : {}", controls.print_unfinished.display_quot()));
        frame.push(format!("Delete                     : {}", controls.delete.display_quot()));
    }

    fn settings_menu(&mut self, controls: &Controls){
        let mut frame = self.frame.borrow_mut();

        frame.push(format!("Settings changed in this menu will only be set when \"Apply Changes\" is selected."));
        frame.push(format!("\nControls:"));
        frame.push(format!("Exit         : {}", controls.escape.display_quot()));
        frame.push(format!("Pointer Up   : {}", controls.up.display_quot()));
        frame.push(format!("Pointer Down : {}", controls.down.display_quot()));
        frame.push(format!("Select       : {}", controls.select.display_quot()));
    }
}