use crate::snake::state::{Direction, State};
use crate::snake::ui;

use crossterm::event::{Event, EventStream, KeyCode};
use futures::{FutureExt, StreamExt};
use rand::Rng;
use std::time::Duration;
use tokio::select;
use tokio::time;
use tokio_stream::wrappers::IntervalStream;

const TICK_MS: u64 = 200;

fn init(mut state: State) -> State {
    state = ui::init(state);

    state = place_snake(state);
    state = place_food(state);

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

    let interval = time::interval(Duration::from_millis(TICK_MS));
    let mut interval_stream = IntervalStream::new(interval);
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().fuse();

        select! {
            _ = interval_stream.next() => {
                state = run_turn(state);
                state = ui::draw_screen(state);
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        // println!("Event::{:?}\r", event);

                        // TODO: could probably filter on event class prior to dispatching the event - all events go down to the handler logic!
                        state = handle_key(state, event);

                        // if event == Event::Key(KeyCode::Esc.into()) {
                        //     break;
                        // }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }

    _ = fini(state);
}

fn handle_key(mut state: State, key: Event) -> State {
    if key == Event::Key(KeyCode::Up.into()) {
        state.direction = Direction::UP;
    };
    if key == Event::Key(KeyCode::Down.into()) {
        state.direction = Direction::DOWN;
    };
    if key == Event::Key(KeyCode::Left.into()) {
        state.direction = Direction::LEFT;
    };
    if key == Event::Key(KeyCode::Right.into()) {
        state.direction = Direction::RIGHT;
    };
    state
}

fn run_turn(mut state: State) -> State {
    let next_head = next_snake_head(state.snake.get(0).unwrap().clone(), state.direction.clone());
    state = move_snake(state, next_head);
    state
}

fn place_snake(mut state: State) -> State {
    state.snake.push((state.width / 2, state.height / 2));
    state
}

fn grow_snake(mut state: State, next_head: (i32, i32)) -> State {
    state.snake.insert(0, next_head);
    state
}

fn move_snake(mut state: State, next_head: (i32, i32)) -> State {
    let _ = state.snake.pop();
    state.snake.insert(0, next_head);
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

fn next_snake_head(current_head: (i32, i32), direction: Direction) -> (i32, i32) {
    let head = current_head;
    let mut x_delta = 0;
    let mut y_delta = 0;
    match direction {
        Direction::UP => y_delta -= 1,
        Direction::DOWN => y_delta += 1,
        Direction::LEFT => x_delta -= 1,
        Direction::RIGHT => x_delta += 1,
    }

    (head.0 + x_delta, head.1 + y_delta)
}
