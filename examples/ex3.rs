use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand, Result};
use std::io::{stdout, Write};

pub struct EdgeSymbols {
    t: char,
    b: char,
    l: char,
    r: char,
    tl: char,
    tr: char,
    bl: char,
    br: char,
}

impl EdgeSymbols {
    pub fn default() -> EdgeSymbols {
        EdgeSymbols {
            t: '─',
            b: '─',
            l: '│',
            r: '│',
            tl: '┌',
            tr: '┐',
            bl: '└',
            br: '┘',
        }
    }
}

fn draw_box(
    width: u16,
    height: u16,
    offset_x: u16,
    offset_y: u16,
    edge_symbols: EdgeSymbols,
) -> Result<()> {
    let mut stdout = stdout();

    let height = height + offset_y;
    let width = width + offset_x;
    let base_x = offset_x;
    let base_y = offset_y;

    for y in offset_y..height {
        for x in offset_x..width {
            if x == base_x && y == base_y {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.tl))?;
            } else if x == base_x && y == height - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.bl))?;
            } else if x == width - 1 && y == base_y {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.tr))?;
            } else if x == width - 1 && y == height - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.br))?;
            } else {
                if x == base_x {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(edge_symbols.l))?;
                } else if x == width - 1 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(edge_symbols.r))?;
                } else if y == base_y {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(edge_symbols.t))?;
                } else if y == height - 1 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(edge_symbols.b))?;
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let w = 80;
    let h = 24;

    _ = draw_box(w, h, 0, 0, EdgeSymbols::default());

    Ok(())
}
