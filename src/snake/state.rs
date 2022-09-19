use ncurses::WINDOW;
use std::ptr;

#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub game_win: WINDOW,
    pub snake: Box<Vec<(i32, i32)>>,
    pub direction: Direction,
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
            direction: Direction::DOWN,
            food: None,
            score: 0,
        }
    }
}
