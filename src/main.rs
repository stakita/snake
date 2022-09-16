pub mod snake;
// pub mod ui;

use crate::snake::state::State;
use crate::snake::ui::init;

// use ncurses;
// use std::{thread, time};

fn main() {
    // println!("Hello, world!");
    // let win = ncurses::initscr();
    // ncurses::noecho();
    // ncurses::cbreak();
    // ncurses::keypad(win, true);
    // let res = ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    // println!("res: {:?}", res);
    // // ncurses::clear();
    // thread::sleep(time::Duration::from_millis(1000));

    // ncurses::wmove(win, 5, 5);
    // ncurses::waddstr(win, ".");
    // ncurses::wmove(win, 6, 6);
    // ncurses::waddstr(win, ".");
    // ncurses::wmove(win, 7, 7);
    // ncurses::waddstr(win, ".");

    // ncurses::refresh();

    // thread::sleep(time::Duration::from_millis(5000));

    // ncurses::endwin();
    let state = State {
        width: 80,
        height: 24,
    };
    init(state);
}
