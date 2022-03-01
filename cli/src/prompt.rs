use std::io::{stdout, stdin};

use frames::*;
use super::Colors;

use crossterm::{terminal, ExecutableCommand};

//use super::control::*;
//use super::display::Display;


struct PromptUpdate{}

impl SizeUpdate for PromptUpdate {
    fn size_update(&mut self, new_size: &Coord, pos: &mut Coord, size: &mut Coord, _offset: &mut Coord, _enabled: &mut bool){
        pos.y = new_size.y - 3;
        size.x = new_size.x;
    }
}

pub struct Prompt{
    object: Rc<RefCell<Object>>,
    frame:  Rc<RefCell<frame_types::Text>>,
}

impl Prompt {
    pub fn new(manager: &mut Manager, colors: &Colors) -> Prompt {
        let frame = frame_types::Text::new(PixelData::new(' ', colors.default, colors.background));
        let obj = Object::new_basic(frame.clone(), Coord{x: manager.get_size().x, y: 3});

        obj.borrow_mut().size_update = Some(Box::new(PromptUpdate{}));
        frame.borrow_mut().push(String::new());

        manager.objects.push(obj.clone());

        Prompt{
            object: obj,
            frame: frame,
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

    pub fn set_prompt(&mut self, manager: &mut Manager, prompt: String) {
        {
            let mut frame = self.frame.borrow_mut();
            frame.set_text(0, prompt);
        }

        manager.add_task(self.object.borrow().update());
    }

    pub fn get_string(&mut self, manager: &mut Manager, prompt: String) -> String {
        String::from(self.get_string_no_trim(manager, prompt).trim())
    }

    pub fn get_string_no_trim(&mut self, manager: &mut Manager, mut prompt: String) -> String {
        prompt.push_str("\n: ");
        self.set_prompt(manager, prompt);
        manager.draw().unwrap();

        let Coord{x, y} = self.object.borrow().pos + Coord{x: 2, y: 1};

        terminal::disable_raw_mode().unwrap();
        stdout().execute(crossterm::cursor::MoveTo(x as u16, y as u16)).unwrap();

        let mut text = String::new();
        stdin().read_line(&mut text).unwrap();

        terminal::enable_raw_mode().unwrap();

        String::from(text)
    }
}

/*
pub fn new_prompt(fill: PixelData, controls: &ControlSettings) -> std::rc::Rc<std::cell::RefCell<frame_types::Text>> {
    let prompt_frame = frame_types::Text::new(fill);
    {
        let mut prompt_conf = prompt_frame.borrow_mut();

        prompt_conf.add_entry("\n\nEnter name of new todo list.\n:");
        prompt_conf.add_entry("\n\nEnter name of new item.\n:");
        prompt_conf.add_entry("\n\nEnter name of todo list to load or nothing to make a new one.\n:");
        prompt_conf.add_entry("\nFile not found.\nEnter name of todo list to load or nothing to make a new one.\n:");
        prompt_conf.add_entry(&entry_controls(controls));
        prompt_conf.add_entry("\n\nEnter new name of item.\n:");
        prompt_conf.add_entry(&config_controls(controls));
        prompt_conf.add_entry("\n\nPress key to change control\n:");
        prompt_conf.add_entry("\n\nSet red value 0-255\n:");
        prompt_conf.add_entry("\n\nSet green value 0-255\n:");
        prompt_conf.add_entry("\n\nSet blue value 0-255\n:");
    }
    
    prompt_frame
}

pub fn entry_controls(controls: &ControlSettings) -> String{
    format!{
    "{}: move up    {}: move down    {}: increase depth    {}: decrease depth    {}: unfinished to txt\n\
    {}: add entry    {}: remove entry    {}: check entry    {}: save and exit    {}: save to txt\n\
    {}: set focus    {}: return focus    {}: save    {}: edit item    {}: config menu",
    controls.key_at_control(Control::Up).display(),
    controls.key_at_control(Control::Down).display(),
    controls.key_at_control(Control::Right).display(),
    controls.key_at_control(Control::Left).display(),
    controls.key_at_control(Control::PrintUnfinished).display(),
    controls.key_at_control(Control::Select).display(),
    controls.key_at_control(Control::Del).display(),
    controls.key_at_control(Control::Complete).display(),
    controls.key_at_control(Control::Esc).display(),
    controls.key_at_control(Control::PrintAll).display(),
    controls.key_at_control(Control::SetRoot).display(),
    controls.key_at_control(Control::BackRoot).display(),
    controls.key_at_control(Control::Save).display(),
    controls.key_at_control(Control::Edit).display(),
    controls.key_at_control(Control::Config).display(),
    }
}

pub fn config_controls(controls: &ControlSettings) -> String{
    format!{
    "\n{}: move up    {}: move down    {}: change key    {}: reset key    {} or {}: exit menu\n",
    controls.key_at_control(Control::Up).display(),
    controls.key_at_control(Control::Down).display(),
    controls.key_at_control(Control::Select).display(),
    controls.key_at_control(Control::Del).display(),
    controls.key_at_control(Control::Esc).display(),
    controls.key_at_control(Control::Config).display(),
    }
}
*/