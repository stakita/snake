use ncurses;
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let _win = ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::clear();

    ncurses::mvaddch(5, 5, '.' as u32);
    ncurses::mvaddch(6, 6, '.' as u32);
    ncurses::mvaddch(7, 7, '.' as u32);

    ncurses::refresh();

    thread::sleep(time::Duration::from_millis(5000));

    ncurses::endwin();
}
