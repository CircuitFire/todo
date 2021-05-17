use frames::*;
use super::control::*;
use super::display::Display;

pub enum Prompt{
    NewList,
    NewItem,
    LoadList,
    LoadError,
    EntryControls,
    EditName,
    ConfigControls,
    SetControl,
    SetR,
    SetG,
    SetB,
}

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