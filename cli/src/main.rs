use std::env::args;
use std::path::PathBuf;
use cli::Todo;

struct Arg {
    reset: bool,
    file: Option<PathBuf>
}

fn handle_args() -> Arg {
    let mut out = Arg {
        reset: false,
        file: None,
    };

    for arg in args() {
        if arg == "-r" {
            out.reset = true;
            continue
        }

        let path = PathBuf::from(arg);

        if path.is_file() {
            out.file = Some(path);
            continue
        }
    }

    out
}

fn main(){
    let arg = handle_args();

    

    let mut todo = Todo::new(arg.reset, arg.file).unwrap();
    todo.main();
}