//use frames::*;
//use crossterm::event::{read, Event};
//
//mod prompt;
//use prompt::*;
//
//mod ui;
//use ui::*;
//
//mod control;
//use control::*;
//
//mod listui;
//
//mod configman;
//use configman::*;
//
//mod display;

mod settings;
use settings::*;

mod list_ui;
use list_ui::*;

mod item_list;
use item_list::*;

mod prompt;
use prompt::Prompt;

mod display;
use display::*;

mod todo;
use todo::*;
pub use todo::Todo;

use frames::{SizeUpdate, Coord};

pub struct MainUiUpdate{}

impl SizeUpdate for MainUiUpdate {
    fn size_update(&mut self, new_size: &Coord, _pos: &mut Coord, size: &mut Coord, _offset: &mut Coord, _enabled: &mut bool){
        *size = *new_size - Coord{x: 0, y: 3};
    }
}