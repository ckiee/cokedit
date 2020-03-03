extern crate crossterm;
extern crate platform_dirs;
extern crate structopt;
#[macro_use]
extern crate lalrpop_util;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crossterm::terminal;
use platform_dirs::{AppDirs, AppUI};
use structopt::StructOpt;

mod ast;
mod consts;
mod keys;
#[cfg(test)]
mod tests;
mod tui;
mod util;

use util::{Editor, Opt};

fn main() {
	let catch_unwind = std::panic::catch_unwind(util::cleanup);
	assert!(catch_unwind.is_ok());
	// Parse args and initalize
	let opt = Opt::from_args();
	let app_dirs = AppDirs::new(Some("cokedit"), AppUI::CommandLine).unwrap();
	let mut editor = Editor {
		buf: String::new(),
		opt,
		scroll: 0,
		pos: 0,
		keybinds: HashMap::new(),
	};
	util::config_to_editor(&app_dirs.config_dir, &mut editor).unwrap();
	let mut file = File::open(&editor.opt.file).unwrap();
	file.read_to_string(&mut editor.buf).unwrap();
	terminal::enable_raw_mode().unwrap();
	tui::draw(&mut editor).unwrap();
	loop {
		keys::handle_keys(&mut editor).unwrap();
		tui::draw(&mut editor).unwrap();
	}
}
