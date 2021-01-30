use ncurses as nc;

mod core;
mod ui;

const KEY_P: i32 = 112;
const KEY_R: i32 = 114;

fn main() {
    nc::setlocale(nc::LcCategory::all, "");
    nc::initscr();
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nc::keypad(nc::stdscr(), true);
    nc::timeout(100);
    ui::init_color_pairs();

    let inner_screen = ui::create_playground();
    let mut game = core::Game::new();

    game.init_snake();

    loop {
        nc::erase();
        nc::werase(inner_screen);
        nc::box_(inner_screen, 0, 0);

        ui::draw_snake(inner_screen, &game.snake);
        game.handle_food();
        ui::draw_food(inner_screen, &game.food);
        ui::draw_score(inner_screen, game.score);

        nc::refresh();
        nc::wrefresh(inner_screen);

        let next_step = game.get_next_step();

        if !game.snake_about_to_collide(next_step) {
            game.snake.crawl(next_step, game.paused);
        } else {
            break;
        }

        let user_input = nc::getch();

        match user_input {
            nc::KEY_UP => {
                game.snake.set_direction(core::Direction::Up);
            }
            nc::KEY_DOWN => {
                game.snake.set_direction(core::Direction::Down);
            }
            nc::KEY_LEFT => {
                game.snake.set_direction(core::Direction::Left);
            }
            nc::KEY_RIGHT => {
                game.snake.set_direction(core::Direction::Right);
            }
            KEY_P => {
                game.paused = !game.paused;
            }
            KEY_R => {
                game.restart();
            }
            _ => {}
        };
    }
    nc::endwin();
}
