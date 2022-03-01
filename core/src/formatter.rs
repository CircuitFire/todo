use super::Entry;
use serde::{Serialize, Deserialize};

pub trait Format{
    fn format(&self, entry: &Entry, depth: usize, has_child: bool) -> String;
}

#[derive(Clone)]
pub struct Formatter {
    indent: usize,
    completed: char,
    incomplete: char,
    formatter: InternalFormatter,
}

impl Formatter {
    pub fn new(indent: usize, completed: char, incomplete: char, form_type: Type) -> Self {
        Formatter {
            formatter: form_type.into_formatter(indent, completed, incomplete),
            indent: indent,
            completed: completed,
            incomplete: incomplete,
        }
    }

    pub fn default() -> Self {
        Formatter::new(1, 'X', ' ', Type::Fancy)
    }

    pub fn get_indent(&self) -> usize {
        self.indent
    }

    pub fn get_completed(&self) -> char {
        self.completed
    }

    pub fn get_incomplete(&self) -> char {
        self.incomplete
    }

    pub fn get_type(&self) -> Type {
        self.formatter.get_type()
    }

    pub fn set(&mut self, indent: Option<usize>, completed: Option<char>, incomplete: Option<char>, form_type: Option<Type>) {
        let mut change = false;

        if let Some(indent) = indent {
            self.indent = indent;
            change = true;
        }

        if let Some(completed) = completed {
            self.completed = completed;
            change = true;
        }

        if let Some(incomplete) = incomplete {
            self.incomplete = incomplete;
            change = true;
        }

        if change || form_type.is_some() {
            let new_type = if let Some(form_type) = form_type {
                form_type
            }
            else {
                self.formatter.get_type()
            };

            self.formatter = new_type.into_formatter(self.indent, self.completed, self.incomplete);
        }
    }
}

impl Format for Formatter {
    fn format(&self, entry: &Entry, depth: usize, has_child: bool) -> String {
        use InternalFormatter::*;

        match self.formatter {
            Basic(ref x) => x.format(entry, depth, has_child),
            Fancy(ref x) => x.format(entry, depth, has_child),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Type {
    Basic,
    Fancy,
}

impl Type {
    fn into_formatter(self, indent: usize, completed: char, incomplete: char) -> InternalFormatter {
        use InternalFormatter::*;

        match self {
            Type::Basic => Basic(BasicForm::new(indent, completed, incomplete)),
            Type::Fancy => Fancy(FancyForm::new(indent, completed, incomplete)),
        }
    }
}

#[derive(Clone)]
enum InternalFormatter {
    Basic(BasicForm),
    Fancy(FancyForm),
}

impl InternalFormatter {
    fn get_type(&self) -> Type {
        use InternalFormatter::*;

        match self {
            Basic(_) => Type::Basic,
            Fancy(_) => Type::Fancy,
        }
    }
}

#[derive(Clone)]
pub struct BasicForm{
    indent: String,
    completed: char,
    incomplete: char,
}

impl BasicForm{
    pub fn new(indent: usize, completed: char, incomplete: char) -> Self {
        BasicForm{
            indent: format!("{}", " ".repeat(indent)),
            completed: completed,
            incomplete: incomplete,
        }
    }

    fn comp(&self, complete: bool) -> char {
        if complete{
            self.completed
        }
        else{
            self.incomplete
        }
    }
}

impl Format for BasicForm{
    fn format(&self, entry: &Entry, depth: usize, _has_child: bool) -> String{
        format!("{}[{}]: {}",
            self.indent.repeat(depth),
            self.comp(entry.complete),
            entry.name,
        )
    }
}

#[derive(Clone)]
pub struct FancyForm{
    connecter: String,
    padding: String,
    completed: char,
    incomplete: char,
}

impl FancyForm{
    pub fn new(indent: usize, completed: char, incomplete: char) -> Self {
        FancyForm{
            connecter: format!("├{}", "─".repeat(indent)),
            padding: format!("│{}", " ".repeat(indent)),
            completed: completed,
            incomplete: incomplete,
        }
    }

    fn comp(&self, complete: bool) -> char {
        if complete{
            self.completed
        }
        else{
            self.incomplete
        }
    }

    fn pipes(&self, depth: usize, has_child: bool) -> String {
        if depth == 0 {
            String::from("┌")
        }
        else{
            format!("{}{}{}",
                self.padding(depth),
                self.connecter,
                self.child_connecter(has_child),
            )
        }
    }

    fn child_connecter(&self, has_child: bool) -> char {
        if has_child  {return '┬'}
        '─'
    }

    fn padding(&self, depth: usize) -> String {
        if depth > 1 {
            format!("{}", self.padding.repeat(depth - 1))
        }
        else{
            String::new()
        }
    }
}

impl Format for FancyForm{
    fn format(&self, entry: &Entry, depth: usize, has_child: bool) -> String {
        format!("{}[{}]: {}",
            self.pipes(depth, has_child),
            self.comp(entry.complete),
            entry.name,
        )
    }
}


/*
┌[]: 
├─┬[]:
│ ├──[]:
│ ├─┬[]:
│ │ ├─[]: 
├──[]:
├──[]:
*/