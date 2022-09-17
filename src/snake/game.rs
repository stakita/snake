use crate::snake::state::State;
use crate::snake::ui; //{draw_screen, fini, init};

use rand::Rng;
use std::{thread, time};

pub fn init(mut state: State) {
    state = ui::init(state);

    state = place_snake(state);
    state = place_food(state);

    println!("state: {:?}", state);

    state = ui::draw_screen(state);
    thread::sleep(time::Duration::from_millis(5000));
    state = ui::fini(state);
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
