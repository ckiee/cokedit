use crate::ast::{Expr, KeyBind, MoveDirection, MoveType, Statement};
use crossterm::{cursor, terminal, QueueableCommand};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Result, Write};
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

lalrpop_mod!(pub config);

pub fn exec_expr(expr: Expr, editor: &mut Editor) {
	match expr {
		Expr::Exit(n) => exit(n),
		Expr::Move(MoveDirection::Next, MoveType::Char) => {
			if editor.pos != editor.buf.chars().count() {
				editor.pos += 1;
			}
		}
		Expr::Move(MoveDirection::Last, MoveType::Char) => {
			if editor.pos != 0 {
				editor.pos -= 1;
			}
		}
		Expr::DeleteOffset(offset) => {
			let tdl = editor.pos - offset;
			if tdl < editor.buf.chars().count() {
				editor.buf.remove(tdl);
				editor.pos -= offset;
			}
		}
		_ => panic!("unimplemented expression type"),
	}
}

pub fn read_config(file: &Path) -> Result<Vec<Statement>> {
	let mut f = File::open(file)?;
	let mut buffer = String::new();
	f.read_to_string(&mut buffer)?;
	let parser = config::StatementParser::new();
	Ok(buffer
		.lines()
		.filter(|l| !l.starts_with("#"))
		.filter(|l| !l.trim().is_empty())
		.map(|l| parser.parse(l).unwrap())
		.collect())
}
pub fn config_to_editor(folder: &PathBuf, editor: &mut Editor) -> Result<()> {
	let mut file = folder.clone();
	file.push("config");
	fs::create_dir_all(folder)?;
	if !file.exists() {
		let mut file = File::create(&file).unwrap();
		file.write_all(&crate::consts::DEFAULT.as_bytes()).unwrap();
		file.flush().unwrap();
	};
	let stmts = read_config(&file)?;
	for stmt in stmts {
		match stmt {
			Statement::Bindsym(kb, exp) => editor.keybinds.insert(kb, exp),
			_ => panic!("Statement Unimplemented"),
		};
	}
	Ok(())
}
pub fn cleanup() {
	terminal::disable_raw_mode().unwrap();
}
pub fn exit(code: i32) {
	cleanup();
	let mut stdout = std::io::stdout();
	stdout.queue(cursor::MoveTo(0, 0)).unwrap();
	stdout
		.queue(terminal::Clear(terminal::ClearType::All))
		.unwrap();
	stdout.flush().unwrap();
	std::process::exit(code);
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "cokedit")]
pub struct Opt {
	#[structopt(name = "FILE", parse(from_os_str))]
	pub file: PathBuf,
}
#[derive(Clone)]
pub struct Editor {
	pub buf: String,
	pub opt: Opt,
	pub scroll: usize,
	pub pos: usize,
	pub keybinds: HashMap<KeyBind, Expr>,
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
