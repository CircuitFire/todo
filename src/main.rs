use todo::*;
use frames::crossterm;
use crossterm::{ExecutableCommand};
use std::io::{stdout};

fn main() -> Result<(), std::io::Error>{
    stdout().execute(crossterm::terminal::EnterAlternateScreen).unwrap();
    let mut todo = App::new()?;
    todo.main();
    stdout().execute(crossterm::terminal::LeaveAlternateScreen).unwrap();
    Ok(())
}