use crate::util;

use crossterm::{
    event,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};

pub fn handle_keys() -> Result<()> {
    match event::read()? {
        Event::Key(k) => match k {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            } => util::exit(0),
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => println!("char: {}", ch),
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
