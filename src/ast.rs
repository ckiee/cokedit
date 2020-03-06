use crossterm::event::KeyModifiers;
use std::hash::Hash;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Statement {
	Expr(Expr),
	// bindsym CONTROL+A <expr>
	Bindsym(KeyBind, Expr),
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	// exit 0
	Exit(i32),
	// exec "ssh boo.local poweroff"
	Exec(String),
	// next/last line/char
	Move(MoveDirection, MoveType),

	DeleteOffset(usize),
}
#[derive(PartialEq, Debug, Clone)]
pub enum MoveDirection {
	Next,
	Last,
}
#[derive(Debug, PartialEq, Clone)]
pub enum MoveType {
	Char,
	Line,
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct KeyBind(pub KeyModifiers, pub char);
