use crossterm::terminal;
use std::path::PathBuf;
use structopt::StructOpt;

pub fn cleanup() {
	terminal::disable_raw_mode().unwrap();
	// std::io::stdout()
	//     .execute(terminal::Clear(terminal::ClearType::All))
	//     .unwrap();
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
	pub pos: usize,
}

impl Editor {
	pub fn get_cursor(&self) -> (usize, usize) {
		self.pos_to_cursor(&self.pos)
	}
	/// (usize, usize) = (col, row) = (x, y)
	pub fn pos_to_cursor(&self, pos: &usize) -> (usize, usize) {
		let mut col = 0;
		let mut row = 0;
		for i in 0..*pos {
			if self.buf.chars().skip(i).next().unwrap() == '\n' {
				row += 1;
				col = 0;
			} else {
				col += 1
			}
		}
		(col, row)
	}
}
