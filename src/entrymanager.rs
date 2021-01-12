use frames::*;

use super::Entry;
use super::Ui;
use super::listui::ListUi;
use super::prompt::*;

use std::fs::File;

pub struct EntryManager{
    root_entry: Rc<RefCell<Entry>>,
    current_root: Vec<Rc<RefCell<Entry>>>,
    depth: usize,
    entry_ui: ListUi,
}

impl EntryManager{
    pub fn new(object: Rc<RefCell<Object>>, frame: Rc<RefCell<frame_types::Text>>) -> EntryManager {
        let entry = Entry::new(String::new());

        EntryManager{
            root_entry: entry.clone(),
            current_root: vec![entry],
            depth: 1,
            entry_ui: ListUi::new(object, frame),
        }
    }

    fn current_root(&self) -> &Rc<RefCell<Entry>>{
        self.current_root.last().unwrap()
    }

    pub fn change_root(&mut self, ui: &mut Ui){
        let (entry, _) = self.entry_at_pointer();
        self.current_root.push(entry);
        self.entry_ui.change_pointer(0, &ui.colors);
        self.entry_ui.format_all_entries(&self.current_root.last().unwrap(), self.depth, &ui.colors);
        ui.update_entries();
        ui.draw();
    }

    pub fn prev_root(&mut self, ui: &mut Ui){
        if self.current_root.len() > 1 {
            self.current_root.pop();
            self.entry_ui.format_all_entries(&self.current_root.last().unwrap(), self.depth, &ui.colors);
            ui.update_entries();
            ui.draw();
        }
    }

    pub fn new_task(&mut self, ui: &mut Ui){
        ui.set_prompt(Prompt::NewItem);
        ui.draw();

        let entry = Entry::new(ui.read_user_text());

        let (found, depth) = self.entry_at_pointer();

        self.entry_ui.format_entry(self.depth, &found, &entry, depth + 1);
        found.borrow_mut().sub_entries.push(entry);
        
        ui.update_entries();
        ui.set_prompt(Prompt::EntryControls);
        ui.draw();
    }

    pub fn new_list(&mut self, ui: &mut Ui){
        ui.set_prompt(Prompt::NewList);
        ui.draw();

        {
            let mut top = self.root_entry.borrow_mut();
            top.name = ui.read_user_text();
            top.completed = false;
            top.sub_entries.clear();
        }

        self.entry_ui.format_all_entries(&self.root_entry, self.depth, &ui.colors);

        ui.update_entries();
        ui.set_prompt(Prompt::EntryControls);
        ui.draw();
    }

    pub fn inc_depth(&mut self, ui: &mut Ui, inc: usize){
        self.depth += inc;

        self.entry_ui.format_all_entries(&self.current_root.last().unwrap(), self.depth, &ui.colors);
        ui.update_entries();
        ui.draw();
    }

    pub fn dec_depth(&mut self, ui: &mut Ui, mut dec: usize){
        if dec > self.depth - 1 {
            dec = self.depth - 1;
        }
        self.depth -= dec;
        
        self.entry_ui.format_all_entries(&self.current_root.last().unwrap(), self.depth, &ui.colors);
        ui.update_entries();
        ui.draw();
    }

    fn entry_at_pointer(&mut self) -> (Rc<RefCell<Entry>>, usize) {
        if self.entry_ui.pointer() == 0 {
            (self.current_root().clone(), 0)
        }
        else{
            let mut pointer = self.entry_ui.pointer();
            if let Some(found) = self.current_root().borrow().entry_at(self.depth - 1, &mut pointer, 1){
                found
            }
            else{
                (self.current_root().clone(), 0)
            }
        }
    }

    pub fn toggle_complete(&mut self, ui: &mut Ui) {
        let mut entry;
        let (t_entry, _) = self.entry_at_pointer();
        entry = t_entry;

        entry.borrow_mut().toggle_comp();
        while !Rc::ptr_eq(&entry, &self.root_entry){
            entry = self.parent_of_entry(&entry);
            if !entry.borrow_mut().toggle_on_children() {
                break;
            }
        } 

        self.entry_ui.format_all_entries(&self.current_root.last().unwrap(), self.depth, &ui.colors);
        ui.update_entries();
        ui.draw();
    }

    pub fn save(&self) {
        self.root_entry.borrow().save().unwrap();
    }

    pub fn load_list(&mut self, ui: &mut Ui) -> Result<bool, std::io::Error> {
        ui.set_prompt(Prompt::LoadList);
        ui.draw();

        loop{
            let mut input = ui.read_user_text();
            if input.is_empty() { return Ok(true)}
            let result = self.root_entry.borrow_mut().load(&mut input);
            if let Err(err) = result {
                match err.kind() {
                    std::io::ErrorKind::NotFound => {
                        ui.set_prompt(Prompt::LoadError);
                        ui.draw();
                    },
                    _ => return Err(err)
                }
            }
            else{
                self.entry_ui.format_all_entries(&self.current_root.last().unwrap(), self.depth, &ui.colors);
                
                ui.update_entries();
                ui.set_prompt(Prompt::EntryControls);
                ui.draw();
                return Ok(false)
            }
        }
    }

    fn parent_of_pointer(&self) -> Option<(Rc<RefCell<Entry>>, usize)> {
        let mut pos = self.entry_ui.pointer();
        if let Some(ret) = Entry::has_child_at_pointer(&self.current_root(), self.depth, &mut pos) {
            if let Some(found) = ret {
                Some(found)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn parent_of_entry(&self, target: &Rc<RefCell<Entry>>) -> Rc<RefCell<Entry>> {
        Entry::has_child(&self.root_entry, target).unwrap()
    }

    pub fn delete_entry(&mut self, ui: &mut Ui) -> bool{
        if (self.entry_ui.pointer() == 0) && Rc::ptr_eq(&self.current_root(), &self.root_entry){
            true
        }
        else{
            if let Some((entry, i)) = self.parent_of_pointer() {
                entry.borrow_mut().sub_entries.remove(i);
                    self.entry_ui.format_all_entries(&self.current_root.last().unwrap(), self.depth, &ui.colors);

                    ui.update_entries();
                    ui.draw();
                    false
            }
            else{
                false
            }
        }
    }

    pub fn print_formatted(&self) -> Result<(), std::io::Error>{
        let mut file = File::create(format!("{}.txt", self.current_root().borrow().name))?;

        self.current_root().borrow().print_formatted(&mut file, self.depth, 0)
    }

    pub fn print_unfinished(&self) -> Result<(), std::io::Error>{
        let mut file = File::create(format!("{}.txt", self.current_root().borrow().name))?;

        self.current_root().borrow().print_unfinished(&mut file, self.depth, 0)
    }

    pub fn edit(&mut self, ui: &mut Ui) {
        ui.set_prompt(Prompt::EditName);
        ui.draw();
        let (entry, depth) = self.entry_at_pointer();

        entry.borrow_mut().set_name(ui.read_user_text());

        self.entry_ui.update_entry(&entry, depth);
        ui.set_prompt(Prompt::EntryControls);
        ui.update_entries();
        ui.draw();
    }

    pub fn inc_pointer(&mut self, ui: &mut Ui, inc: usize){
        self.entry_ui.inc_pointer(ui, inc);
        ui.update_entries();
        ui.draw();
    }

    pub fn dec_pointer(&mut self, ui: &mut Ui, dec: usize){
        self.entry_ui.dec_pointer(ui, dec);
        ui.update_entries();
        ui.draw();
    }

    pub fn refresh_colors(&mut self, colors: &[Color]){
        self.entry_ui.refresh_colors(colors);
    }
}