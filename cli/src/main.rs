use std::env::{Args, args};
use cli::Todo;

fn main(){
    let mut reset = false;

    for arg in args() {
        if arg == "-r" {
            reset = true;
            break
        }
    }

    let mut todo = Todo::new(reset).unwrap();
    todo.main();
}