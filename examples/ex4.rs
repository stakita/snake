use crossterm::{
    cursor::{self, position},
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute, queue, style, terminal, Result,
};
use std::io::{self, Write};

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn test_event<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    // Only needed if you want terminal mouse capture
    execute!(w, EnableMouseCapture)?;

    loop {
        // Blocking read
        let event = read()?;

        println!("Event::{:?}\r", event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }

        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }

    // Only needed if you enabled terminal mouse capture
    execute!(w, DisableMouseCapture)?;

    Ok(())
}

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    // Only needed if you want an alternate screen
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    queue!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(1, 1),
        cursor::Show,
        // cursor::EnableBlinking
        cursor::Hide,
    )?;

    test_event(w)?;

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        // Only needed if you used an alternate screen
        terminal::LeaveAlternateScreen
    )?;

    _ = terminal::disable_raw_mode();

    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}
