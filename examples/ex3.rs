use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand, Result};
use std::{
    collections::HashMap,
    io::{stdout, Write},
};

fn main() -> Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut border = HashMap::new();
    border.insert("t", '─');
    border.insert("b", '─');
    border.insert("l", '|');
    border.insert("r", '|');
    border.insert("tl", '┌');
    border.insert("tr", '┐');
    border.insert("bl", '└');
    border.insert("br", '┘');

    let w = 150;
    let h = 40;

    for y in 0..h {
        for x in 0..w {
            if x == 0 && y == 0 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(border["tl"]))?;
            } else if x == 0 && y == h - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(border["bl"]))?;
            } else if x == w - 1 && y == 0 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(border["tr"]))?;
            } else if x == w - 1 && y == h - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(border["br"]))?;
            } else {
                if x == 0 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(border["l"]))?;
                } else if x == w - 1 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(border["r"]))?;
                } else if y == 0 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(border["t"]))?;
                } else if y == h - 1 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(border["b"]))?;
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
