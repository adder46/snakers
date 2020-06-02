use ncurses::*;

mod core;
mod ui;

fn main() {
    setlocale(LcCategory::all, "");
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(100);

    let mut game = core::Game::new(LINES(), COLS() / 2);
    game.snake().init_body(LINES() / 2, COLS() / 4);

    loop {
        erase();

        ui::draw_snake(&game.snake());
        game.handle_food();
        ui::draw_food(&game.food());

        refresh();

        game.snake().crawl();

        let user_input = getch();
        match user_input {
            KEY_UP => { game.snake().set_direction(core::Direction::Up); },
            KEY_DOWN => { game.snake().set_direction(core::Direction::Down); },
            KEY_LEFT => { game.snake().set_direction(core::Direction::Left); },
            KEY_RIGHT => { game.snake().set_direction(core::Direction::Right); },
            _ => {}
        };
    }
    endwin();
}
