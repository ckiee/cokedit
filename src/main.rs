extern crate crossterm;
extern crate structopt;

use std::fs::File;
use std::io::Read;

use crossterm::terminal;
use structopt::StructOpt;

mod keys;
mod tui;
mod util;

use util::{Editor, Opt};

fn main() {
    let catch_unwind = std::panic::catch_unwind(util::cleanup);
    assert!(catch_unwind.is_ok());
    // Parse args and initalize
    let opt = Opt::from_args();
    terminal::enable_raw_mode().unwrap();
    let mut editor = Editor {
        buf: String::new(),
        opt,
        scroll: 0,
        cursor: (0, 0),
    };
    let mut file = File::open(&editor.opt.file).unwrap();
    file.read_to_string(&mut editor.buf).unwrap();

    tui::draw(&mut editor).unwrap();
    loop {
        keys::handle_keys().unwrap();
        tui::draw(&mut editor).unwrap();
    }
}
