use ncurses::WINDOW;
use std::ptr;

#[derive(Debug)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub game_win: WINDOW,
    pub snake: Box<Vec<(i32, i32)>>,
    pub food: Option<(i32, i32)>,
    pub score: i32,
}

impl State {
    pub fn new() -> State {
        State {
            width: 80,
            height: 24,
            game_win: ptr::null_mut(),
            snake: Box::new(Vec::new()),
            food: None,
            score: 0,
        }
    }
}
