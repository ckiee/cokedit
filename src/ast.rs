use crossterm::event::KeyModifiers;
lalrpop_mod!(pub config);
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Statement {
	Expr(Expr),
	// bindsym CONTROL+A <expr>
	Bindsym(KeyBind, Expr),
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Expr {
	// exit 0
	Exit(i32),
	// exec "hello"
	Exec(String),
	// next/last line/char
	Move(MoveDirection, MoveType),
}
#[derive(PartialEq, Debug)]
pub enum MoveDirection {
	Next,
	Last,
}
#[derive(Debug, PartialEq)]
pub enum MoveType {
	Char,
	Line,
}
#[derive(Debug, PartialEq)]
pub struct KeyBind(pub KeyModifiers, pub char);
