//mod entry;
//
//mod formatter;
//pub use formatter::*;
//
//mod buffer;
//
//mod entrymanager;
//pub use entrymanager::EntryManager;

use bytebuffer::*;

use std::{fs, io::Read};
use std::path::PathBuf;

use tree;
pub use tree::*;

mod formatter;
pub use formatter::*;

#[derive(Clone)]
pub struct Entry{
    pub name: String,
    pub complete: bool,
}

impl<'a> IntoBytes<'a> for Entry {
    fn into_bytes(&'a self) -> Box<dyn Iterator<Item = u8> + 'a> {
        Box::new(self.name.as_bytes().into_bytes().chain(self.complete.into_bytes()))
    }
}

impl FromBytes for Entry {
    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr>{
        Ok(Entry{
            name: String::from(String::from_utf8_lossy(&Vec::from_bytes(bytes)?)),
            complete: bool::from_bytes(bytes)?,
        })
    }

    fn from_io_bytes<T: Iterator<Item = Result<u8, std::io::Error>>>(bytes: &mut T) -> Result<Self, ByteErr>{
        Ok(Entry{
            name: String::from(String::from_utf8_lossy(&Vec::from_io_bytes(bytes)?)),
            complete: bool::from_io_bytes(bytes)?,
        })
    }
}

pub struct Core{
    roots: Vec<usize>,
    depth: usize,
    tree: Tree<Entry>,
}

impl Core{
    pub fn new(name: String) -> Core {
        Core{
            tree: Tree::new_with_root(Entry{
                name: name,
                complete: false
            }),
            roots: vec![0],
            depth: 2,
        }
    }

    pub fn get_entry_ids(&self) -> Vec<usize> {
        self.tree.sub_tree_depth(*self.roots.last().unwrap(), self.depth).unwrap()
    }

    pub fn get_entries_info(&self) -> Vec<NodeInfo> {
        self.tree.sub_tree_depth_info(*self.roots.last().unwrap(), self.depth).unwrap()
    }

    pub fn new_entry(&mut self, name: String, pos: Position, id: usize) {
        self.tree.new_node(
            Entry{
                name: name,
                complete: false,
            },
            pos,
            id,
        ).unwrap();
    }

    pub fn get_entry(&self, id: usize) -> &Entry{
        self.tree.data_at(id).unwrap()
    }

    pub fn inc_depth(&mut self, amount: usize) {
        self.depth += amount;
    }

    pub fn dec_depth(&mut self, amount: usize) {
        if amount > self.depth {
            self.depth = 0;
        }
        else {
            self.depth -= amount;
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn parent_id(&self, id: usize) -> usize{
        self.tree.parent_of(id).unwrap().unwrap()
    }

    fn set_comp_down(&mut self, id: usize, new: bool) -> bool {
        let mut data = self.tree.data_at_mut(id).unwrap();
        if data.complete == new { return false }

        data.complete = new;

        for child in self.tree.children_of(id).unwrap(){
            self.set_comp_down(child, new);
        }

        true
    }

    pub fn set_incomplete(&mut self, id: usize){
        if !self.set_comp_down(id, false) { return }
        
        let mut current = id;
        while let Some(parent) = self.tree.parent_of(current).unwrap() {
            let mut data = self.tree.data_at_mut(parent).unwrap();
            if !data.complete { return }
    
            data.complete = false;

            current = parent;
        }
    }

    pub fn set_complete(&mut self, id: usize){
        if !self.set_comp_down(id, true) { return }

        let mut current = id;
        while let Some(parent) = self.tree.parent_of(current).unwrap() {
            if self.tree.data_at(parent).unwrap().complete { return }

            for child in self.tree.children_of(parent).unwrap() {
                if !self.tree.data_at(child).unwrap().complete { return }
            }
    
            self.tree.data_at_mut(parent).unwrap().complete = true;

            current = parent;
        }
    }

    pub fn toggle_comp(&mut self, id: usize) {
        if self.tree.data_at(id).unwrap().complete {
            self.set_incomplete(id);
        }
        else{
            self.set_complete(id);
        }
    }

    pub fn name(&self) -> &str {
        &self.tree.data_at(self.tree.get_root().unwrap()).unwrap().name
    }

    pub fn save<'a>(&'a self) -> Box<dyn Iterator<Item = u8> + 'a>{
        self.tree.into_bytes()
    }

    pub fn load(file_name: &PathBuf) -> Result<Core, ByteErr> {
        let file = fs::OpenOptions::new().read(true).open(file_name)?;
        let tree = Tree::from_io_bytes(&mut file.bytes())?;

        Ok(Core{
            depth: 2,
            roots: vec![tree.get_root().unwrap()],
            tree: tree,
        })
    }

    pub fn delete(&mut self, id: usize) {
        self.tree.remove(id).unwrap();
    }

    pub fn current_root(&self) -> usize {
        *self.roots.last().unwrap()
    }

    pub fn copy_entry(&mut self, cloning: usize, in_position: Position, node: usize) {
        self.tree.clone_to(cloning, in_position, node).unwrap();
    }

    pub fn move_entry(&mut self, cloning: usize, in_position: Position, node: usize) {
        self.tree.move_to(cloning, in_position, node).unwrap();
    }

    pub fn descendants_of(&self, id: usize) -> Vec<usize> {
        self.tree.descendants_of(id).unwrap()
    }
}
