// use crate::snake::state::{Direction, State};
// use crate::snake::ui;
use std::error::Error;

use crossterm::event::{Event, EventStream, KeyCode};
use futures::{FutureExt, StreamExt};
use rand::Rng;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::select;
use tokio::time;
use tokio_stream::wrappers::IntervalStream;

const TICK_MS: u64 = 200;

#[derive(Debug)]
pub struct State {
    pub width: Arc<Mutex<i32>>,
    pub height: Arc<Mutex<i32>>,
    // pub game_win: WINDOW,
    pub snake: Box<Vec<(i32, i32)>>,
    // pub direction: Direction,
    // pub previous: Direction,
    pub food: Option<(i32, i32)>,
    pub game_over: bool,
    pub score: i32,
    pub done: Arc<Mutex<bool>>,
}

impl State {
    pub fn new() -> State {
        State {
            width: Arc::new(Mutex::new(80)),
            height: Arc::new(Mutex::new(24)),
            // game_win: ptr::null_mut(),
            snake: Box::new(Vec::new()),
            // direction: Direction::DOWN,
            // previous: Direction::DOWN,
            food: None,
            game_over: false,
            score: 0,
            done: Arc::new(Mutex::new(false)),
        }
    }
}

fn init(state: Arc<std::sync::Mutex<State>>) {
    println!("init - start");

    let mut state = state.lock().unwrap();

    // let done = Arc::clone(&state.lock().unwrap().done);

    // ctrlc::set_handler(move || {
    //     let mut val = done.lock().unwrap();
    //     *val = true;
    // })
    // .expect("Error setting Ctrl-C handler");

    // state = ui::init(state);

    state = place_snake(state);
    state = place_food(state);
    println!("init - end");
    // state
}

// fn finish() {
//     // ui::finish();
// }

pub async fn ticker(state: Arc<Mutex<State>>) {
    println!("ticker - start");
    let interval = time::interval(Duration::from_millis(TICK_MS * 2));
    let mut interval_stream = IntervalStream::new(interval);

    // init(state);

    let done = false;

    // while !*state.done.lock().unwrap() {
    while !done {
        println!("ticker - loop");
        interval_stream.next().await;
        let mut val = state.lock().unwrap();
        val.score += 1;
        println!("tick - val: {}", val.score);
        // if !state.lock().clone().game_over {
        //     state = run_turn(state);
        //     // ui::draw_screen(&state);
        //     println!("draw_screen");
        // }
        // if state.lock().clone().game_over {
        //     // ui::game_over(&state);
        //     println!("game_over");
        // }
        println!("ticker - loop end");
    }
}

pub async fn run() {
    println!("run - start");

    let mut state = Arc::new(Mutex::new(State::new()));
    let state_clone = Arc::clone(&state);
    init(state_clone);
    // ui::draw_screen(&state);

    println!("run - 1");
    let interval = time::interval(Duration::from_millis(TICK_MS));
    let mut interval_stream = IntervalStream::new(interval);
    // let mut reader = EventStream::new();

    println!("run - 2");
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        ticker(state_clone).await;
    });

    println!("run - 3");
    let done = false;
    // while !*state.done.lock().unwrap() {
    while !done {
        println!("run - loop");
        interval_stream.next().await;
        let mut val = state.lock().unwrap();
        val.score += 1000;
        println!("1 -- tick - val: {}", val.score);
        // if !state.game_over {
        //     state = run_turn(state);
        //     // ui::draw_screen(&state);
        //     println!("1 -- draw_screen");
        // }
        // if state.game_over {
        //     // ui::game_over(&state);
        //     println!("1 -- game_over");
        // }

        // let event = reader.next().fuse();

        // select! {
        //     _ = interval_stream.next() => {
        //         if !state.game_over {
        //             state = run_turn(state);
        //             // ui::draw_screen(&state);
        //             println!("draw_screen");
        //         }
        //         if state.game_over {
        //             // ui::game_over(&state);
        //             println!("game_over");
        //         }
        //     },
        //     maybe_event = event => {
        //         match maybe_event {
        //             Some(Ok(event)) => {
        //                 // TODO: could probably filter on event class prior to dispatching the event - all events go down to the handler logic!
        //                 state = handle_key(state, &event);

        //                 // Exit if 'q' or Esc is pressed
        //                 if event == Event::Key(KeyCode::Esc.into()) || event == Event::Key(KeyCode::Char('q').into()) {
        //                     break;
        //                 }
        //             }
        //             Some(Err(e)) => println!("Error: {:?}\r", e),
        //             None => break,
        //         }
        //     }
        // };
        println!("run - loop end");
    }

    // finish();
}

// fn handle_key(mut state: State, key: &Event) -> State {
//     if key == &Event::Key(KeyCode::Up.into())
//         && state.previous != Direction::DOWN
//         && state.previous != Direction::UP
//     {
//         println!("UP");
//         state.direction = Direction::UP;
//     };
//     if key == &Event::Key(KeyCode::Down.into())
//         && state.previous != Direction::UP
//         && state.previous != Direction::DOWN
//     {
//         println!("DOWN");
//         state.direction = Direction::DOWN;
//     };
//     if key == &Event::Key(KeyCode::Left.into())
//         && state.previous != Direction::RIGHT
//         && state.previous != Direction::LEFT
//     {
//         println!("LEFT");
//         state.direction = Direction::LEFT;
//     };
//     if key == &Event::Key(KeyCode::Right.into())
//         && state.previous != Direction::LEFT
//         && state.previous != Direction::RIGHT
//     {
//         println!("RIGHT");
//         state.direction = Direction::RIGHT;
//     };
//     state
// }

// fn run_turn(mut state: State) -> State {
//     let next_head = next_snake_head(state.snake.get(0).unwrap().clone(), state.direction.clone());

//     state.previous = state.direction.clone();

//     if loses(&state, next_head) {
//         state.game_over = true;
//     } else if hits_food(&state, next_head) {
//         state = grow_snake(state, next_head);
//         state = place_food(state);
//         state = incr_score(state);
//     } else {
//         state = move_snake(state, next_head);
//     }
//     state
// }

fn place_snake(mut state: std::sync::MutexGuard<'_, State>) -> std::sync::MutexGuard<'_, State> {
    // let done = &state.lock().unwrap();
    println!("place_snake - start");
    // let mut state = state.lock().unwrap();
    // // println!("place_snake - 1");
    // // let width = &state.lock().unwrap().width;
    // // println!("place_snake - 2");
    // // let height = &state.lock().unwrap().height;

    // // state_inner
    // //     .snake
    // //     .push((state_inner.width / 2, state_inner.height / 2));
    // println!("place_snake - 3");

    // state.snake.push((state.width / 2, &state.height / 2));
    let height = *state.height.lock().unwrap();
    let width = *state.width.lock().unwrap();

    state.snake.push((width / 2, height / 2));
    println!("place_snake - end");

    state
}

// fn grow_snake(mut state: State, next_head: (i32, i32)) -> State {
//     state.snake.insert(0, next_head);
//     state
// }

// fn incr_score(mut state: State) -> State {
//     state.score += 1;
//     state
// }

// fn move_snake(mut state: State, next_head: (i32, i32)) -> State {
//     let _ = state.snake.pop();
//     state.snake.insert(0, next_head);
//     state
// }

fn place_food(mut state: std::sync::MutexGuard<'_, State>) -> std::sync::MutexGuard<'_, State> {
    let mut done: bool = false;
    let height = *state.height.lock().unwrap();
    let width = *state.width.lock().unwrap();
    while !done {
        let location = (
            rand::thread_rng().gen_range(1..=width - 2),
            rand::thread_rng().gen_range(1..=height - 3),
        );

        if !hits_snake(&state.snake, location) {
            state.food = Some(location);
            done = true;
        }
    }
    state
}

// fn loses(state: &State, next_head: (i32, i32)) -> bool {
//     hits_wall(state, next_head) || hits_snake(&state.snake, next_head)
// }

// fn hits_wall(state: &State, next_head: (i32, i32)) -> bool {
//     next_head.0 == 0
//         || next_head.1 == 0
//         || next_head.0 == state.width - 1
//         || next_head.1 == state.height - 2
// }

fn hits_snake(snake: &Box<Vec<(i32, i32)>>, location: (i32, i32)) -> bool {
    snake.contains(&location)
}

// fn hits_food(state: &State, next_head: (i32, i32)) -> bool {
//     state.food.unwrap() == next_head
// }

// fn next_snake_head(current_head: (i32, i32), direction: Direction) -> (i32, i32) {
//     let head = current_head;
//     let mut x_delta = 0;
//     let mut y_delta = 0;
//     match direction {
//         Direction::UP => y_delta -= 1,
//         Direction::DOWN => y_delta += 1,
//         Direction::LEFT => x_delta -= 1,
//         Direction::RIGHT => x_delta += 1,
//     }

//     (head.0 + x_delta, head.1 + y_delta)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await;
    Ok(())
}
