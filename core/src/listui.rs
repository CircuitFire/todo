use frames::*;

use super::entry::Entry;
use super::ui::Ui;

pub struct ListUi {
    object: Rc<RefCell<Object>>,
    frame: Rc<RefCell<frame_types::Text>>,
    pointer: usize,
}

impl ListUi {
    pub fn new(object: Rc<RefCell<Object>>, frame: Rc<RefCell<frame_types::Text>>) -> ListUi {
        ListUi{
            object: object,
            frame: frame,
            pointer: 0,
        }
    }

    pub fn pointer(&self) -> usize{
        self.pointer
    }

    pub fn color_pointer(&mut self, pointer: usize, colors: &[Color]){
        self.frame.borrow_mut().set_entry_fg(pointer, colors[1]);
        self.pointer = pointer;
        self.offset();
    }

    pub fn change_pointer(&mut self, new: usize, colors: &[Color]){
        self.frame.borrow_mut().set_entry_fg(self.pointer, colors[0]);
        self.frame.borrow_mut().set_entry_fg(new, colors[1]);
        self.pointer = new;
        self.offset();
    }

    pub fn inc_pointer(&mut self, ui: &mut Ui, inc: usize){
        let new = (self.pointer + inc) % self.get_entry_len();

        self.change_pointer(new, &ui.colors);
    }

    pub fn dec_pointer(&mut self, ui: &mut Ui, dec: usize){
        let size = self.get_entry_len();
        let new = (self.pointer + (size - (dec % size))) % size;
        
        self.change_pointer(new, &ui.colors);
    }

    pub fn update_entry(&mut self, entry: &Rc<RefCell<Entry>>, depth: usize){
        let mut entry_frame = self.frame.borrow_mut();

        entry_frame.set_entry_text(self.pointer, &entry.borrow().to_string(depth));
    }

    pub fn format_entry(&mut self, max_depth: usize, parent: &Rc<RefCell<Entry>>, entry: &Rc<RefCell<Entry>>, entry_depth: usize){
        let mut entry_frame = self.frame.borrow_mut();

        if entry_depth <= max_depth {

            if self.pointer == entry_frame.entries_len() {
                entry_frame.add_entry(&entry.borrow().to_string(max_depth));
            }
            else{
                let pointer = self.pointer + parent.borrow().count(max_depth + 1 - entry_depth);
                entry_frame.insert_entry(pointer, &entry.borrow().to_string(entry_depth));
            }
        }
    }

    pub fn format_all_entries(&mut self, entry: &Rc<RefCell<Entry>>, depth: usize, colors: &[Color]){
        self.frame.borrow_mut().clear_entries();
        self.format_helper(entry, depth, 0);

        let len = self.frame.borrow(). entries_len();
        
        if self.pointer >= len {
            self.pointer = len - 1;
        }
        self.color_pointer(self.pointer, colors);
    }

    fn format_helper(&mut self, entry: &Rc<RefCell<Entry>>, max_depth: usize, cur_depth: usize){
        self.frame.borrow_mut().add_entry(&entry.borrow().to_string(cur_depth));

        if max_depth != 0 {
            for sub_entry in &entry.borrow().sub_entries{
                self.format_helper(sub_entry, max_depth - 1, cur_depth + 1);
            } 
        }
    }

    pub fn get_entry_len(&self) -> usize {
        self.frame.borrow().entries_len()
    }

    fn offset(&mut self){
        let obj_size = self.object.borrow().get_size().y;
        let entry_len = self.frame.borrow().entries_len();
        let half = obj_size / 2;

        if (obj_size < entry_len as i32) && (self.pointer > half as usize) {
            
            let offset = if self.pointer > (entry_len - (half as usize + 1))
            {
                entry_len as i32 - obj_size
            }
            else{
                self.pointer as i32 - half
            };

            self.object.borrow_mut().set_offset(&Coord{x: 0, y: offset})
        }
        else{
            self.object.borrow_mut().set_offset(&Coord{x: 0, y: 0})
        }
    }

    pub fn add_entry(&mut self, text: &str){
        self.frame.borrow_mut().add_entry(text);
    }

    pub fn set_entry_text(&mut self, index: usize, text: &str){
        self.frame.borrow_mut().set_entry_text(index, text);
    }

    pub fn refresh_colors(&mut self, colors: &[Color]){
        self.frame.borrow_mut().set_fill(PixelData::new(' ', colors[0], Color::Rgb{r: 0, g: 0, b: 0}));

        for i in 0..self.get_entry_len(){
            self.frame.borrow_mut().set_entry_fg(i, colors[0]);
        }

        self.frame.borrow_mut().set_entry_fg(self.pointer(), colors[1]);
    }
}