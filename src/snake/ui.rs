use crate::snake::state::State;

use ncurses;

pub fn init(state: State) -> State {
    println!("state: {:?}", state);
    ncurses::initscr();

    let win = ncurses::newwin(state.height - 1, state.width, 1, 0);
    ncurses::raw();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(win, true);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE); // This actually doesn't work for some reason

    State {
        height: state.height,
        width: state.width,
        game_win: win,
    }
}

pub fn fini(state: State) -> State {
    ncurses::endwin();
    state
}

pub fn draw_screen(state: State) -> State {
    //   def draw_screen(state) do
    //     ExNcurses.clear()
    ncurses::clear();
    //     ExNcurses.mvaddstr(0, 2, "Snake")
    ncurses::mvaddstr(0, 2, "Snake");
    //     ExNcurses.wclear(state.game_win)
    ncurses::wclear(state.game_win);
    //     ExNcurses.wborder(state.game_win)
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
    //     update_score(state)
    //     draw_snake(state, state.snake)
    //     draw_food(state)
    //     ExNcurses.refresh()
    ncurses::refresh();
    //     ExNcurses.wrefresh(state.game_win)
    ncurses::wrefresh(state.game_win);
    //     state
    //   end
    state
}

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
