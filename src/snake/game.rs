use crate::snake::state::State;
use crate::snake::ui; //{draw_screen, fini, init};

use crossterm::cursor::position;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::{FutureExt, StreamExt};
use rand::Rng;
use std::time::Duration;
// use std::{thread, time};
use tokio::select;
use tokio::time::sleep;
// use futures_timer::Delay;

const TICK_MS: u64 = 200;

fn init(mut state: State) -> State {
    state = ui::init(state);

    state = place_snake(state);
    state = place_food(state);

    println!("state: {:?}", state);

    state
}

fn fini(mut state: State) -> State {
    state = ui::fini(state);
    state
}

pub async fn run() {
    let mut state = State::new();
    state = init(state);
    state = ui::draw_screen(state);

    let mut reader = EventStream::new();

    loop {
        let delay = sleep(Duration::from_millis(TICK_MS));
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

    _ = fini(state);
}

fn place_snake(mut state: State) -> State {
    state.snake.push((state.width / 2, state.height / 2));
    state
}

fn place_food(mut state: State) -> State {
    // let location = (
    //     rand::thread_rng().gen_range(1..=state.width - 2),
    //     rand::thread_rng().gen_range(1..=state.height - 2),
    // );

    // if hits_snake(state.snake, location) {
    //     place_food(state)
    // } else {
    //     state.food = Some(location);
    //     state
    // }

    let mut done: bool = false;
    while !done {
        let location = (
            rand::thread_rng().gen_range(1..=state.width - 2),
            rand::thread_rng().gen_range(1..=state.height - 3),
        );

        if !hits_snake(&state.snake, location) {
            state.food = Some(location);
            done = true;
        }
    }
    state
}

fn hits_snake(snake: &Box<Vec<(i32, i32)>>, location: (i32, i32)) -> bool {
    snake.contains(&location)
}
