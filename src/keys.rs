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
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } if editor.pos != 0 => {
                editor.buf.remove(editor.pos - 1);
            }
            KeyEvent {
                code: KeyCode::Delete,
                ..
            } if editor.pos != editor.buf.chars().count() - 1 => {
                editor.buf.remove(editor.pos);
            }
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => editor.pos += 1,
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => editor.pos -= 1,
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
