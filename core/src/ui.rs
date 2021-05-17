use frames::*;

use super::prompt::*;

use std::io::{stdin, stdout, Write};
use crossterm::{ExecutableCommand, QueueableCommand};

use super::control::*;

use std::io::prelude::*;
use std::fs::File;

use super::buffer::*;
use super::display::Display;

use std::fs::OpenOptions;

pub enum Mode{
    Entry,
    Config,
}

pub struct Ui {
    pub manager: Manager,
    prompt_obj: Rc<RefCell<Object>>,
    prompt_frame: Rc<RefCell<frame_types::Text>>,
    entry_obj: Rc<RefCell<Object>>,
    entry_frame: Rc<RefCell<frame_types::Text>>,
    config_obj: Rc<RefCell<Object>>,
    config_frame: Rc<RefCell<frame_types::Text>>,
    pub colors: [Color; 2],
}

impl Ui {
    pub fn new(controls: &ControlSettings) -> Result<Ui, std::io::Error> {
        let (x, y) = crossterm::terminal::size().unwrap();
        let size = Coord { x: x as i32, y: y as i32 };

        let colors = Ui::load_colors();

        let mut manager = Manager::new(size, &Pixel::new(' ', colors[0], Color::Rgb{r: 0, g: 0, b: 0})).unwrap();

        let entry_frame = frame_types::Text::new(PixelData::new(' ', colors[0], Color::Rgb{r: 0, g: 0, b: 0}));
        let entry_obj = Object::new_basic(entry_frame.clone(), Coord{x: 0, y: 0}, Coord{x: size.x, y: size.y - 4});
        manager.objects().push(entry_obj.clone());

        let prompt_frame = new_prompt(PixelData::new(' ', colors[0], Color::Rgb{r: 0, g: 0, b: 0}), controls);
        let prompt_obj = Object::new_basic(prompt_frame.clone(), Coord{x: 0, y: size.y - 4}, Coord{x: size.x, y: 4});
        manager.objects().push(prompt_obj.clone());

        let config_frame = frame_types::Text::new(PixelData::new(' ', colors[0], Color::Rgb{r: 0, g: 0, b: 0}));
        let config_obj = Object::new_basic(config_frame.clone(), Coord{x: 0, y: 0}, Coord{x: 0, y: 0});
        manager.objects().push(config_obj.clone());

        manager.draw().unwrap();

        Ok(Ui{
            manager: manager,
            prompt_obj: prompt_obj,
            prompt_frame: prompt_frame,
            entry_obj: entry_obj,
            entry_frame: entry_frame,
            config_obj: config_obj,
            config_frame: config_frame,
            colors: colors,
        })
    }

    pub fn get_config_obj(&self) -> Rc<RefCell<Object>> {
        self.config_obj.clone()
    }

    pub fn get_entry_obj(&self) -> Rc<RefCell<Object>> {
        self.entry_obj.clone()
    }

    pub fn get_config_frame(&self) -> Rc<RefCell<frame_types::Text>> {
        self.config_frame.clone()
    }

    pub fn get_entry_frame(&self) -> Rc<RefCell<frame_types::Text>> {
        self.entry_frame.clone()
    }

    pub fn load_colors() -> [Color; 2] {
        if let Ok(mut file) = File::open("todo.cnf"){
            let mut buffer = [0; 3];

            file.read(&mut buffer).unwrap();
            let normal_color = buff_to_color(&buffer);
            file.read(&mut buffer).unwrap();

            [normal_color, buff_to_color(&buffer)]
        }
        else {
            [Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 255, b: 255}]
        }
    }

    pub fn save_colors(&self) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).truncate(false).create(true).open("todo.cnf")?;
        let mut buffer = [0; 6];

        self.colors[0].into_buffer(&mut buffer[..3]);
        self.colors[1].into_buffer(&mut buffer[3..6]);
        
        file.write(&buffer)?;

        Ok(())
    }

    pub fn draw(&mut self){
        self.manager.draw().unwrap();
    }

    pub fn resize(&mut self, size: &Coord) {
        self.manager.set_size(size);

        self.entry_obj.borrow_mut().set_size(&Coord{x: size.x, y: size.y - 3});

        self.prompt_obj.borrow_mut().set_pos(&Coord{x: 0, y: size.y - 3});
        self.prompt_obj.borrow_mut().set_size(&Coord{x: size.x, y: 3});

        self.manager.draw().unwrap();
    }

    pub fn set_prompt(&mut self, prompt: Prompt){
        self.prompt_obj.borrow_mut().set_offset(&Coord{x: 0, y: prompt as i32});
        self.manager.add_task(self.prompt_obj.borrow().update());
    }

    pub fn update_entries(&mut self){
        self.manager.add_task(self.entry_obj.borrow().update());
    }

    pub fn update_config(&mut self){
        self.manager.add_task(self.config_obj.borrow().update());
    }

    pub fn read_user_text(&self) -> String {
        let size = self.manager.get_size();

        crossterm::terminal::disable_raw_mode().unwrap();
        stdout().queue(crossterm::cursor::Show).unwrap();
        stdout().queue(crossterm::cursor::MoveTo(1, (size.y - 1) as u16)).unwrap();
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut input = input.trim().to_string();
        input.truncate(50);

        stdout().execute(crossterm::cursor::Hide).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();

        input
    }

    pub fn read_user_u8(&self) -> u8 {
        let size = self.manager.get_size();

        crossterm::terminal::disable_raw_mode().unwrap();
        stdout().queue(crossterm::cursor::Show).unwrap();
        stdout().queue(crossterm::cursor::MoveTo(1, (size.y - 1) as u16)).unwrap();
        stdout().flush().unwrap();

        let mut input = String::new();
        let num;
        loop{
            input.clear();
            stdin().read_line(&mut input).unwrap();
            if let Ok(temp) = input.trim().parse::<u8>(){
                num = temp;
                break;
            }
        }
        
        stdout().execute(crossterm::cursor::Hide).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();

        num
    }

    pub fn set_mode(&mut self, mode: Mode){
        let (x, y) = crossterm::terminal::size().unwrap();
        let size = Coord { x: x as i32, y: y as i32 };

        match mode {
            Mode::Entry => {
                self.entry_obj.borrow_mut().set_size(&Coord{x: size.x, y: size.y - 4});
                self.config_obj.borrow_mut().set_size(&Coord{x: 0, y: 0});
                self.update_entries();
            },
            Mode::Config => {
                self.config_obj.borrow_mut().set_size(&Coord{x: size.x, y: size.y - 4});
                self.entry_obj.borrow_mut().set_size(&Coord{x: 0, y: 0});
                self.update_config();
            }
        }
    }

    pub fn update_prompts(&mut self, controls: &ControlSettings){
        let mut prompt = self.prompt_frame.borrow_mut();
        prompt.set_entry_text(4, &entry_controls(&controls));
        prompt.set_entry_text(6, &config_controls(&controls));
        self.manager.add_task(self.prompt_obj.borrow().update());
    }

    pub fn color_string(&self, index: usize) -> String{
        match index{
            0 => format!("Normal Color   : {}", self.colors[index].display()),
            1 => format!("Highlight Color: {}", self.colors[index].display()),
            _ => String::new(),
        }
    }

    pub fn refresh_colors(&self){
        let mut prompt = self.prompt_frame.borrow_mut();

        for i in 0..prompt.entries_len(){
            prompt.set_entry_fg(i, self.colors[0]);
        }
    }

    pub fn default_color(&mut self, index: usize){
        match index{
            0 => self.colors[0] = Color::Rgb{r: 255, g: 255, b: 255},
            1 => self.colors[1] = Color::Rgb{r: 0, g: 255, b: 255},
            _ => (),
        }
    }
}