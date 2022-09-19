use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand, Result};
use std::{
    collections::HashMap,
    io::{stdout, Write},
};

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

    for y in offset_y..height {
        for x in offset_x..width {
            if x == 0 && y == 0 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.tl))?;
            } else if x == 0 && y == height - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.bl))?;
            } else if x == width - 1 && y == 0 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.tr))?;
            } else if x == width - 1 && y == height - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(edge_symbols.br))?;
            } else {
                if x == 0 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(edge_symbols.l))?;
                } else if x == width - 1 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(edge_symbols.r))?;
                } else if y == 0 {
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

    let w = 150;
    let h = 40;
    let use_draw_box = false;

    if use_draw_box {
        _ = draw_box(w, h, 0, 0, EdgeSymbols::default());
    } else {
        let mut border = HashMap::new();
        border.insert("t", '─');
        border.insert("b", '─');
        border.insert("l", '│');
        border.insert("r", '│');
        border.insert("tl", '┌');
        border.insert("tr", '┐');
        border.insert("bl", '└');
        border.insert("br", '┘');

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
    }
    Ok(())
}
