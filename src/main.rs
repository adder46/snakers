use ncurses::*;

mod core;
mod ui;

fn main() {
    setlocale(LcCategory::all, "");
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(100);
    ui::init_color_pairs();

    let inner_screen = ui::create_playground();
    let mut game = core::Game::new();

    game.init_snake();

    loop {
        erase();
        werase(inner_screen);
        box_(inner_screen, 0, 0);

        ui::draw_snake(inner_screen, &game.snake);
        game.handle_food();
        ui::draw_food(inner_screen, &game.food);

        refresh();
        wrefresh(inner_screen);

        let next_step = game.get_next_step();

        if !game.snake_about_to_collide(&next_step) {
            game.snake.crawl(&next_step);
        } else {
            break;
        }

        let user_input = getch();

        match user_input {
            KEY_UP => {
                game.snake.set_direction(core::Direction::Up);
            }
            KEY_DOWN => {
                game.snake.set_direction(core::Direction::Down);
            }
            KEY_LEFT => {
                game.snake.set_direction(core::Direction::Left);
            }
            KEY_RIGHT => {
                game.snake.set_direction(core::Direction::Right);
            }
            _ => {}
        };
    }
    endwin();
}
