use crate::snake::state::State;

use ncurses;

pub fn init(state: State) -> State {
    let _ = &println!("state: {:?}", state);
    ncurses::initscr();

    let win = ncurses::newwin(state.height - 1, state.width, 1, 0);
    ncurses::raw();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(win, true);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE); // This actually doesn't work for some reason

    State {
        game_win: win,
        ..state
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
    let state = update_score(state);
    draw_snake(&state);
    draw_food(&state);
    //     ExNcurses.refresh()

    // We can't hide the cursor on macos, so just park it in the corner
    // ncurses::wmove(state.game_win, 0, 0);

    ncurses::refresh();
    //     ExNcurses.wrefresh(state.game_win)
    ncurses::wrefresh(state.game_win);
    //     state
    //   end
    state
}

//   defp draw_food(state) do
//     {x, y} = state.food
//     ExNcurses.wmove(state.game_win, y, x)
//     ExNcurses.waddstr(state.game_win, "*")
//     state
//   end
fn draw_food(state: &State) {
    let (x, y) = state.food.unwrap();
    ncurses::wmove(state.game_win, y, x);
    ncurses::waddstr(state.game_win, "*");
}

//   defp draw_snake(state, []), do: state

//   defp draw_snake(state, [{x, y} | rest]) do
//     ExNcurses.wmove(state.game_win, y, x)
//     ExNcurses.waddstr(state.game_win, "#")
//     draw_snake(state, rest)
//   end
fn draw_snake(state: &State) {
    let snake_iter = state.snake.iter();
    for elem in snake_iter {
        let (x, y) = elem;
        ncurses::wmove(state.game_win, *y, *x);
        ncurses::waddstr(state.game_win, "#");
    }
    // draw_snake(state, rest)
}

fn update_score(state: State) -> State {
    let score_str = format!("Score: {}", state.score);
    ncurses::mvaddstr(0, state.width - 20, &score_str);
    state
}
