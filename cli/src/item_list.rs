use frames::*;

use core::{Core, NodeInfo, Position, Format};
use super::*;

use std::{fs, io::Write};
use std::path::PathBuf;
use bytebuffer::*;

pub enum ItemControls {
    Esc,
    PointerUp,
    PointerDown,
    Select,
    IncDepth,
    DecDepth,
    Toggle,
    Save,
    Help,
    Print,
    PrintUnfinished,
    Delete,
    Move,
    Copy,
}

pub enum Selection {
    Esc,
    PointerUp,
    PointerDown,
    Select,
}

pub struct Data {
    info: Vec<NodeInfo>,
    core: Core,
}

pub struct ItemList {
    ui: ListUi,
}

impl ItemList {
    pub fn new(manager: &mut Manager, colors: &Colors) -> ItemList {
        let mut ui = ListUi::new(manager, Some(Box::new(MainUiUpdate{})), colors);
        ui.disable();

        ItemList{
            ui: ui,
        }
    }

    pub fn new_list(&mut self, name: String) -> Data{
        //self.ui.pointer = 0;
        self.ui.set_pointer_no_update(0);
        let core = Core::new(name);

        Data{
            info: core.get_entries_info(),
            core: core,
        }
    }

    pub fn load_list(&mut self, name: &PathBuf) -> Result<Data, ByteErr> {
        //self.ui.pointer = 0;
        self.ui.set_pointer_no_update(0);
        let core = Core::load(name)?;

        Ok(Data{
            info: core.get_entries_info(),
            core: core,
        })
    }

    pub fn main(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager, prompt: &mut Prompt) {
        use ItemControls::*;

        self.ui.enable();
        self.format_entries(data, settings, manager);
        prompt.set_prompt(manager, settings.controls().help_tip());

        loop {
            manager.draw().unwrap();

            match settings.controls().item(manager) {
                Esc             => break,
                PointerUp       => self.pointer_up(manager, settings),
                PointerDown     => self.pointer_down(manager, settings),
                Select          => self.select(data, settings, manager, prompt),
                IncDepth        => self.inc_depth(data, settings, manager),
                DecDepth        => self.dec_depth(data, settings, manager),
                Toggle          => self.toggle(data, settings, manager),
                Save            => save(data),
                Help            => self.help(settings, manager, prompt),
                Print           => self.print(data, settings),
                PrintUnfinished => self.print_unfinished(data, settings),
                Delete          => self.delete(data, settings, manager),
                Move            => self.move_entry(data, settings, manager, prompt),
                Copy            => self.copy_entry(data, settings, manager, prompt),
            }
        }

        self.ui.disable();
    }

    fn pointer_up(&mut self, manager: &mut Manager, settings: &mut Settings) {
        self.ui.dec_pointer(manager, &settings.colors(), 1);
    }

    fn pointer_down(&mut self, manager: &mut Manager, settings: &mut Settings) {
        self.ui.inc_pointer(manager, &settings.colors(), 1);
    }

    fn format_entries(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager) {
        self.ui.clear();
        let formatter = settings.formatter();

        for item in &data.info {
            self.ui.push(formatter.format(data.core.get_entry(item.id), item.depth, item.child_count > 0))
        }

        self.ui.refresh_colors(&settings.colors(), manager);
    }

    fn select(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager, prompt: &mut Prompt) {
        let name = prompt.get_string(manager, String::from("Name of new child"));
        data.core.new_entry(name, Position::LastChild, data.info[self.ui.pointer()].id);

        data.info = data.core.get_entries_info();
        
        self.format_entries(data, settings, manager);
        prompt.set_prompt(manager, settings.controls().item_prompt());
    }

    fn find_new_id_pos(&mut self, id: usize, data: &mut Data, settings: &mut Settings, manager: &mut Manager) {
        data.info = data.core.get_entries_info();

        for (i, node) in data.info.iter().enumerate(){
            if node.id == id {
                //self.ui.pointer = i;
                self.ui.set_pointer_no_update(i);
                break;
            }
        }

        self.format_entries(data, settings, manager);
    }

    fn inc_depth(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager){
        data.core.inc_depth(1);
        let id = data.info[self.ui.pointer()].id;

        self.find_new_id_pos(id, data, settings, manager);
    }

    fn dec_depth(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager){
        data.core.dec_depth(1);
        let current = data.info[self.ui.pointer()];

        let id = if current.depth > data.core.depth() {
            data.core.parent_id(current.id)
        }
        else {
            current.id
        };

        self.find_new_id_pos(id, data, settings, manager);
    }

    fn toggle(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager) {
        data.core.toggle_comp(data.info[self.ui.pointer()].id);
        data.info = data.core.get_entries_info();
        self.format_entries(data, settings, manager);
    }

    fn help(&mut self, settings: &mut Settings, manager: &mut Manager, prompt: &mut Prompt) {
        self.ui.disable();
        settings.help_menu(manager, prompt, HelpMenuType::Item);
        self.ui.enable();
        self.ui.update(manager);
        prompt.set_prompt(manager, settings.controls().item_prompt());
    }

    fn print(&mut self, data: &mut Data, settings: &mut Settings) {
        let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(format!("./{}.txt", data.core.name())).unwrap();
        let formatter = settings.print_formatter();

        for item in &data.info {
            write!(file, "{}\n", formatter.format(&data.core.get_entry(item.id), item.depth, item.child_count > 0)).unwrap();
        }
    }

    fn print_unfinished(&mut self, data: &mut Data, settings: &mut Settings) {
        let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(format!("./{}.txt", data.core.name())).unwrap();
        let formatter = settings.print_formatter();

        for item in &data.info {
            let entry = data.core.get_entry(item.id);

            if !entry.complete {
                write!(file, "{}\n", formatter.format(entry, item.depth, item.child_count > 0)).unwrap();
            }
        }
    }

    fn delete(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager){
        if data.info[self.ui.pointer()].id != data.core.current_root() {
            data.core.delete(data.info[self.ui.pointer()].id);

            self.find_new_id_pos(data.info[self.ui.pointer() - 1].id, data, settings, manager);
        }
    }

    fn get_position(&mut self, settings: &mut Settings, manager: &mut Manager, prompt: &mut Prompt) -> Position {
        use Position::*;

        if self.ui.pointer() == 0 {
            prompt.set_prompt(manager, settings.controls().position_root_prompt());
        }
        else{
            prompt.set_prompt(manager, settings.controls().position_prompt());
        }

        manager.draw().unwrap();
        
        loop {
            let pos = settings.controls().position(manager);
            if (pos == SiblingBefore) || (pos == SiblingAfter) {
                if self.ui.pointer() != 0 { continue }
            }

            return pos;
        }
    }

    fn multi_select(&mut self, settings: &mut Settings, manager: &mut Manager, prompt: &mut Prompt, check_children: Option<SubTree>) -> Option<(usize, Position, usize)> {
        use Selection::*;

        self.ui.update(manager);
        prompt.set_prompt(manager, settings.controls().select_prompt());

        let ret;

        loop {
            manager.draw().unwrap();

            match settings.controls().multi_select(manager) {
                Esc             => {
                    ret = None;
                    break;
                }
                PointerUp       => self.pointer_up(manager, settings),
                PointerDown     => self.pointer_down(manager, settings),
                Select          => {
                    if let Some(children) = &check_children {
                        if children.is_child(self.ui.pointer()) { continue }
                    }

                    if self.ui.pointer() != self.ui.selected().unwrap() {
                        ret = Some((
                            self.ui.selected().unwrap(),
                            self.get_position(settings, manager, prompt),
                            self.ui.pointer(),
                        ));
                        break;
                    }
                }
            }
        }

        prompt.set_prompt(manager, settings.controls().help_tip());
        self.ui.deselect();
        return ret
    }

    fn move_entry(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager, prompt: &mut Prompt){
        if self.ui.pointer() != 0 {
            self.ui.select();
            let children = SubTree::new(data.core.descendants_of(self.ui.selected().unwrap()), &data.info);

            if let Some(temp) = self.multi_select(settings, manager, prompt, Some(children)){
                data.core.move_entry(data.info[temp.0].id, temp.1, data.info[temp.2].id);
                self.find_new_id_pos(data.info[self.ui.pointer()].id, data, settings, manager);
            }
            else{
                self.ui.refresh_colors(&settings.colors(), manager);
                manager.draw().unwrap();
            }
        }
    }

    fn copy_entry(&mut self, data: &mut Data, settings: &mut Settings, manager: &mut Manager, prompt: &mut Prompt){
        self.ui.select();
        
        if let Some(temp) = self.multi_select(settings, manager, prompt, None){
            data.core.copy_entry(data.info[temp.0].id, temp.1, data.info[temp.2].id);
            self.find_new_id_pos(data.info[self.ui.pointer()].id, data, settings, manager);
        }
        else{
            self.ui.refresh_colors(&settings.colors(), manager);
            manager.draw().unwrap();
        }
    }
}

fn save(data: &mut Data) {
    let mut file = fs::OpenOptions::new().write(true).create(true).open(format!("./{}.todo", data.core.name())).unwrap();

    file.write_bytes(&mut data.core.save(), 1024).unwrap();
}

struct SubTree<'a>{
    children: Vec<usize>,
    info: &'a Vec<NodeInfo>,
}

impl<'a> SubTree<'a> {
    fn new(children: Vec<usize>, info: &Vec<NodeInfo>) -> SubTree {
        SubTree {
            children: children,
            info: info
        }
    }

    fn is_child(&self, pointer: usize) -> bool {
        for child in &self.children {
            if self.info[pointer].id == *child {return true}
        }

        return false
    }
}