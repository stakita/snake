use crate::snake::state::State;
use ansi_escapes;
use ncurses;
use std::io::{stdout, Write};

pub fn init(state: State) -> State {
    ncurses::initscr();

    let win = ncurses::newwin(state.height - 1, state.width, 1, 0);
    ncurses::raw();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(win, true);
    hide_cursor();

    State {
        game_win: win,
        ..state
    }
}

pub fn fini(state: State) -> State {
    ncurses::endwin();
    show_cursor();
    state
}

// TODO: handle error instead of suppressing
fn hide_cursor() {
    // ncurses::curs_set doesn't work for some reason
    // Replacing with manual ansi escape code
    let mut stdout = stdout();
    let _ = stdout.write(&format!("{}", ansi_escapes::CursorHide).as_bytes().to_vec());
    let _ = stdout.flush();
}

// TODO: handle error instead of suppressing
fn show_cursor() {
    let mut stdout = stdout();
    let _ = stdout.write(&format!("{}", ansi_escapes::CursorShow).as_bytes().to_vec());
    let _ = stdout.flush();
}

pub fn game_over(state: State) -> State {
    center_text(&state, " GAME OVER ");
    ncurses::refresh();
    // TODO: do we need to flush anything here??
    // flush_input();
    state
}

pub fn draw_screen(state: State) -> State {
    ncurses::clear();
    ncurses::mvaddstr(0, 2, "Snake");
    ncurses::wclear(state.game_win);
    ncurses::wborder(
        state.game_win,
        '|' as u32,
        '|' as u32,
        '-' as u32,
        '-' as u32,
        '+' as u32,
        '+' as u32,
        '+' as u32,
        '+' as u32,
    );
    let state = update_score(state);
    draw_snake(&state);
    draw_food(&state);

    ncurses::refresh();
    ncurses::wrefresh(state.game_win);
    state
}

fn draw_food(state: &State) {
    let (x, y) = state.food.unwrap();
    ncurses::wmove(state.game_win, y, x);
    ncurses::waddstr(state.game_win, "*");
}

fn draw_snake(state: &State) {
    let snake_iter = state.snake.iter();
    for elem in snake_iter {
        let (x, y) = elem;
        ncurses::wmove(state.game_win, *y, *x);
        ncurses::waddstr(state.game_win, "#");
    }
}

fn center_text(state: &State, text: &str) {
    let y = state.height / 2;
    let x = (state.width - text.chars().count() as i32) / 2;
    ncurses::mvaddstr(y, x, text);
}

fn update_score(state: State) -> State {
    let score_str = format!("Score: {}", state.score);
    ncurses::mvaddstr(0, state.width - 20, &score_str);
    state
}
