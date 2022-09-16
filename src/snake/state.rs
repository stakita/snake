use ncurses::WINDOW;

#[derive(Debug)]
pub struct State {
    pub width: i32,
    pub height: i32,
    pub game_win: WINDOW,
}
