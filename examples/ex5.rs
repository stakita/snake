//! Demonstrates how to read events asynchronously with async-std.
//!
//! cargo run --features="event-stream" --example event-stream-async-std

use std::{io::stdout, time::Duration};

use futures::{future::FutureExt, StreamExt};
use tokio::select;
use tokio::time::sleep;

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

const HELP: &str = r#"EventStream based on futures_util::stream::Stream with async-std
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

async fn print_events() {
    let mut reader = EventStream::new();

    loop {
        let delay = sleep(Duration::from_millis(1_000));
        let event = reader.next().fuse();

        select! {
            _ = delay => { println!(".\r"); },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    print_events().await;

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}
