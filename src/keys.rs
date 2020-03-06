use crate::ast::{Expr, MoveDirection, MoveType};
use crate::util;
use crossterm::{
    event,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};

pub fn handle_keys(editor: &mut util::Editor) -> Result<()> {
    match event::read()? {
        Event::Key(k) => match k {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            } => util::exit(0),
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => {
                editor.buf.insert(editor.pos, ch);
                editor.pos += 1;
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => util::exec_expr(Expr::DeleteOffset(1), editor),

            KeyEvent {
                code: KeyCode::Delete,
                ..
            } => util::exec_expr(Expr::DeleteOffset(0), editor),
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => util::exec_expr(Expr::Move(MoveDirection::Next, MoveType::Char), editor),
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => util::exec_expr(Expr::Move(MoveDirection::Last, MoveType::Char), editor),
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
