// Source: https://users.rust-lang.org/t/text-mode-terminal-application-with-asynchronous-input-output/74760

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use futures::stream::StreamExt;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    // Async task that emits log lines periodically and can be controlled:
    let (input_s, mut input_r) = tokio::sync::mpsc::channel::<Option<String>>(100);
    let (output_s, mut output_r) = tokio::sync::mpsc::channel::<String>(100);
    tokio::spawn(async move {
        let mut state = Some(String::from("Hello world"));
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if let Ok(new_state) = input_r.try_recv() {
                state = new_state;
            }
            match state.clone() {
                Some(string) => {
                    output_s
                        .send(format!("{:?} {}", Instant::now(), string))
                        .await
                        .unwrap();
                }
                None => {
                    break;
                }
            }
        }
    });

    // UI state
    let mut log: Vec<String> = Vec::new();
    let mut input_buffer = String::new();

    let mut reader = EventStream::new();
    loop {
        terminal.draw(|f| {
            let [output_size, input_size]: [Rect; 2] = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(f.size())
                .try_into()
                .unwrap();
            let output = Paragraph::new(log.join("\n"))
                .block(Block::default().title("Output").borders(Borders::ALL));
            let input = Paragraph::new(&*input_buffer)
                .block(Block::default().title("Input").borders(Borders::ALL));
            f.render_widget(output, output_size);
            f.render_widget(input, input_size);
        })?;
        tokio::select! {
            log_line = output_r.recv() => {
                // TODO: if the sender is dropped we should stop checking this future
                log.push(log_line.unwrap());
            }

            event_result = reader.next() => {
                let event = match event_result {
                    None => break,
                    Some(Err(_)) => break, // IO error on stdin
                    Some(Ok(event)) => event,
                };
                match event {
                    // Quit
                    Event::Key(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => {
                            break;
                    }
                    // Delete character
                    Event::Key(KeyEvent {
                        code: KeyCode::Backspace, ..
                    }) => {
                        input_buffer.pop();
                    }
                    // Send line
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter, ..
                    }) => {
                        input_s.send(Some(input_buffer.clone())).await.unwrap();
                        input_buffer.clear();
                    }
                    // Type character
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c), ..
                    }) => {
                        input_buffer.push(c);
                    }
                    _ => {
                        // Unrecognized input
                        write!(terminal.backend_mut().by_ref(), "\x07")?;
                        terminal.backend_mut().flush()?;
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
