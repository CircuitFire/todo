use frames::*;

use super::*;

pub struct ListUi {
    object: Rc<RefCell<Object>>,
    frame: Rc<RefCell<frame_types::Text>>,
    pointer: usize,
    selected: Option<usize>,
}

impl ListUi {
    pub fn new(manager: &mut Manager, update: Option<Box<dyn SizeUpdate>>, colors: &Colors) -> ListUi {
        let frame = frame_types::Text::new(PixelData::new(' ', colors.default, colors.background));
        let obj = Object::new_min(frame.clone());
        obj.borrow_mut().size_update = update;
        
        manager.objects.push(obj.clone());

        ListUi{
            object: obj,
            frame: frame,
            pointer: 0,
            selected: None,
        }
    }

    pub fn enable(&mut self) {
        self.object.borrow_mut().enabled = true;
    }

    pub fn disable(&mut self) {
        self.object.borrow_mut().enabled = false;
    }

    pub fn pointer(&self) -> usize{
        self.pointer
    }

    pub fn set_pointer_no_update(&mut self, new: usize) {
        self.pointer = new;
    }

    pub fn select(&mut self) {
        self.selected = Some(self.pointer);
    }

    pub fn selected(&self) -> Option<usize>{
        self.selected
    }

    pub fn deselect(&mut self) {
        self.selected = None;
    }

    pub fn color_pointer(&mut self, colors: &Colors){
        self.frame.borrow_mut().set_fg(self.pointer, colors.pointer);
        self.offset();
    }

    pub fn set_pointer(&mut self, new: usize, manager: &mut Manager, colors: &Colors){
        if self.selected == Some(self.pointer) {
            self.frame.borrow_mut().set_fg(self.pointer, colors.selected);
        }
        else{
            self.frame.borrow_mut().set_fg(self.pointer, colors.default);
        }
        
        self.frame.borrow_mut().set_fg(new, colors.pointer);
        self.pointer = new;
        self.offset();

        self.update(manager);
    }

    pub fn inc_pointer(&mut self, manager: &mut Manager, colors: &Colors, inc: usize){
        let new = (self.pointer + inc) % self.len();

        self.set_pointer(new, manager, colors);
    }

    pub fn dec_pointer(&mut self, manager: &mut Manager, colors: &Colors, dec: usize){
        let size = self.len();
        let new = (self.pointer + (size - (dec % size))) % size;
        
        self.set_pointer(new, manager, colors);
    }

    pub fn len(&self) -> usize {
        self.frame.borrow().len()
    }

    ///If the list is longer than the allocated screen space this will offset the shown list items when the pointer gets to the middle of the list instead of moving the pointer.
    fn offset(&mut self){
        let obj_size = self.object.borrow().size.y;
        let entry_len = self.frame.borrow().len();
        let half = obj_size / 2;

        if (obj_size < entry_len as i32) && (self.pointer > half as usize) {
            
            let offset = if self.pointer > (entry_len - (half as usize + 1))
            {
                entry_len as i32 - obj_size
            }
            else{
                self.pointer as i32 - half
            };

            self.object.borrow_mut().offset = Coord{x: 0, y: offset};
        }
        else{
            self.object.borrow_mut().offset = Coord{x: 0, y: 0};
        }
    }

    pub fn push(&mut self, text: String){
        self.frame.borrow_mut().push(text);
    }

    pub fn set_text(&mut self, index: usize, text: String){
        self.frame.borrow_mut().set_text(index, text);
    }

    pub fn set_fg(&mut self, index: usize, color: Color){
        self.frame.borrow_mut().set_fg(index, color);
    }

    pub fn set_bg(&mut self, index: usize, color: Color){
        self.frame.borrow_mut().set_bg(index, color);
    }

    pub fn frame(&mut self) -> &Rc<RefCell<frame_types::Text>>{
        &self.frame
    }

    ///Reanalyzes what entries should be colored. Used after editing the item list.
    pub fn refresh_colors(&mut self, colors: &Colors, manager: &mut Manager){
        let mut frame = self.frame.borrow_mut();
        frame.fill = PixelData::new(' ', colors.default, colors.background);

        if frame.len() == 0 {return}

        for i in 0..frame.len(){
            frame.set_fg(i, colors.default);
            frame.set_bg(i, colors.background);
        }

        if let Some(selected) = self.selected {
            frame.set_fg(selected, colors.pointer);
        }

        frame.set_fg(self.pointer, colors.pointer);

        self.update(manager);
    }

    ///checks if the pointer is in range and moves it if it is not and refreshes the colors.
    pub fn refresh(&mut self, colors: &Colors, manager: &mut Manager){
        if self.pointer > self.len() {
            self.pointer = self.len();
        }

        self.refresh_colors(colors, manager);
    }

    pub fn clear(&mut self) {
        self.frame.borrow_mut().clear();
    }

    pub fn update(&self, manager: &mut Manager) {
        manager.add_task(self.object.borrow().update());
    }

    pub fn truncate(&mut self, len: usize) {
        self.frame.borrow_mut().truncate(len);
    }
}