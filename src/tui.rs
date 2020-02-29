use crossterm::*;
use std::io::{stdout, Write};

use crate::util::Editor;

pub fn draw(editor: &mut Editor) -> Result<()> {
    let mut stdout = stdout();
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    // editor.buf.lines().count()
    stdout.queue(cursor::MoveTo(0, 0))?;
    let mut ln = 0;
    for line in editor.buf.lines().skip(editor.scroll) {
        ln += 1;
        stdout.queue(style::Print(line))?;
        if ln > terminal::size()?.1 {
            break;
        }
        stdout.queue(cursor::MoveToNextLine(1))?;
    }
    stdout.queue(cursor::MoveTo(0, 0))?;

    stdout.flush()?;
    Ok(())
}
