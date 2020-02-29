use crossterm::{terminal, ExecutableCommand};
use std::path::PathBuf;
use structopt::StructOpt;

pub fn cleanup() {
    terminal::disable_raw_mode().unwrap();
    std::io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}
pub fn exit(code: i32) {
    cleanup();
    std::process::exit(code);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "cokedit")]
pub struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}

pub struct Editor {
    pub buf: String,
    pub opt: Opt,
    pub scroll: usize,
    pub cursor: (usize, usize),
}
