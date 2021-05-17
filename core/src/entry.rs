use std::{rc::Rc, cell::RefCell};
use std::io::prelude::*;
use std::fs::File;

use super::buffer::*;

pub struct Entry{
    pub name: String,
    pub completed: bool,
    pub sub_entries: Vec<Rc<RefCell<Entry>>>,
}

impl Entry {
    pub fn new(name: String) -> Rc<RefCell<Entry>>{
        Rc::new(RefCell::new(
            Entry{
                name: name,
                completed: false,
                sub_entries: Vec::new(),
            }
        ))
    }

    pub fn count(&self, depth: usize) -> usize {
        if depth == 0 {
            1
        }
        else {
            let mut count = 1;

            for entry in &self.sub_entries{
                count += entry.borrow().count(depth - 1);
            }

            count
        }
    }

    pub fn to_string(&self, depth: usize) -> String {
        format!(
            "{}[{}] {}",
            "    ".repeat(depth),
            if self.completed {"X"} else {" "},
            self.name
        )
    }

    pub fn entry_at(&self, max_depth: usize, pos: &mut usize, cur_depth: usize) -> Option<(Rc<RefCell<Entry>>, usize)> {
        let mut found = None;
        for entry in &self.sub_entries {
            *pos -= 1;
            if *pos == 0 {
                found = Some((entry.clone(), cur_depth));
                break;
            }
            else{
                if max_depth != 0 {
                    if let Some(entry) = entry.borrow().entry_at(max_depth - 1, pos, cur_depth + 1){
                        found = Some(entry);
                        break;
                    }
                }
            }
        }
        found
    }

    pub fn has_child_at_pointer(current: &Rc<RefCell<Entry>>, max_depth: usize, pos: &mut usize) -> Option<Option<(Rc<RefCell<Entry>>, usize)>> {
        let mut ret = None;
        if *pos == 0 {
            ret = Some(None);
        }
        else{
            for (i, entry) in current.borrow().sub_entries.iter().enumerate() {
                *pos -= 1;
                if let Some(found) = Entry::has_child_at_pointer(entry, max_depth - 1, pos){
                    match found {
                        Some(entry) => ret = Some(Some(entry)),
                        None => ret = Some(Some((current.clone(), i))),
                    }
                    break;
                }
            }
        }
        ret
    }

    pub fn toggle_comp(&mut self) {
        self.completed = !self.completed;

        for child in &mut self.sub_entries {
            Entry::set_children(child, self.completed);
        }
    }

    fn set_children(entry: &Rc<RefCell<Entry>>, value: bool){
        if entry.borrow().completed != value{
            entry.borrow_mut().completed = value;

            for child in &mut entry.borrow_mut().sub_entries {
                Entry::set_children(child, value);
            }
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut file = File::create(format!("./Lists/{}.todo", self.name))?;

        self.save_helper(&mut file)?;

        Ok(())
    }

    fn save_helper(&self, file: &mut File) -> std::io::Result<()> {
        write!(file, "{:<50}", self.name, )?;
        file.write(&self.completed.as_buffer())?;
        file.write(&(self.sub_entries.len() as u32).as_buffer())?;

        for entry in &self.sub_entries {
            entry.borrow().save_helper(file)?;
        }

        Ok(())
    }

    pub fn load(&mut self, file_name: &mut String) -> std::io::Result<()> {
        let mut file = File::open(format!("./Lists/{}.todo", file_name))?;

        
        let mut buffer = [0; 50];
        file.read(&mut buffer[..50])?;
        self.name = String::from_utf8_lossy(&buffer[..50]).trim().to_string();
        
        self.load_helper(&mut buffer, &mut file)?;

        Ok(())
    }

    fn load_helper(&mut self, buffer: &mut [u8], file: &mut std::fs::File) -> std::io::Result<()> {

        file.read(&mut buffer[..1])?;
        self.completed = buff_to_bool(&buffer[..1]);
        file.read(&mut buffer[..4])?;
        let size = buff_to_u32(&buffer[..4]) as usize;
        self.sub_entries = Vec::with_capacity(size);

        for _ in 0..size {
            file.read(&mut buffer[..50])?;
            let entry = Entry::new(String::from_utf8_lossy(&buffer[..50]).trim().to_string());
            entry.borrow_mut().load_helper(buffer, file)?;
            self.sub_entries.push(entry);
        }
    
        Ok(())
    }

    pub fn toggle_on_children(&mut self) -> bool {
        let mut all_complete = true;

        for child in &self.sub_entries{
            if !child.borrow().completed {
                all_complete = false;
                break;
            }
        }

        if self.completed {
            if all_complete {
                false
            }
            else {
                self.completed = false;
                true
            }
        }
        else{
            if all_complete {
                self.completed = true;
                true
            }
            else {
                false
            }
        }
    }

    pub fn has_child(current: &Rc<RefCell<Entry>>, target: &Rc<RefCell<Entry>>) -> Option<Rc<RefCell<Entry>>> {
        let mut ret = None;
        for entry in &current.borrow().sub_entries {
            if Rc::ptr_eq(&entry, &target) {
                ret = Some(current.clone());
                break;
            }

            if let Some(found) = Entry::has_child(entry, target){
                ret = Some(found);
                break;
            }
        }
        ret
    }

    pub fn print_formatted(&self, file: &mut File, max_depth: usize, cur_depth: usize) -> Result<(), std::io::Error>{
        write!(file, "{}\n", self.to_string(cur_depth))?;

        if cur_depth != max_depth {
            for child in &self.sub_entries{
                child.borrow().print_formatted(file, max_depth, cur_depth + 1)?;
            }
        }

        Ok(())
    }

    pub fn print_unfinished(&self, file: &mut File, max_depth: usize, cur_depth: usize) -> Result<(), std::io::Error>{
        if !self.completed {
            write!(file, "{}\n", self.to_string(cur_depth))?;

            if cur_depth != max_depth {
                for child in &self.sub_entries{
                    child.borrow().print_unfinished(file, max_depth, cur_depth + 1)?;
                }
            }
        }
        Ok(())
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}