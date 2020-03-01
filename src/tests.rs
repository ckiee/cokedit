use crate::ast::*;
use crossterm::event::KeyModifiers;

lalrpop_mod!(pub config);

#[test]
fn config_expr() {
	assert_eq!(
		config::ExprParser::new().parse("exit").unwrap(),
		Expr::Exit(0)
	);
	assert_eq!(
		config::ExprParser::new().parse("exit 69").unwrap(),
		Expr::Exit(69)
	);
	assert_eq!(
		config::ExprParser::new().parse("exec \"true\"").unwrap(),
		Expr::Exec("true".to_string())
	);
	assert_eq!(
		config::ExprParser::new().parse("next char").unwrap(),
		Expr::Move(MoveDirection::Next, MoveType::Char)
	);
}

#[test]
fn config_stmt() {
	assert_eq!(
		config::StatementParser::new()
			.parse("bindsym Ctrl+x exit")
			.unwrap(),
		Statement::Bindsym(KeyBind(KeyModifiers::CONTROL, 'x'), Expr::Exit(0))
	);
}
