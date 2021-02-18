use crate::core::{Food, Snake};
use ncurses as nc;

pub const PLAYGROUND_HEIGHT: i32 = 20;
pub const PLAYGROUND_WIDTH: i32 = 80;

pub fn create_playground() -> nc::WINDOW {
    nc::newwin(
        PLAYGROUND_HEIGHT,
        PLAYGROUND_WIDTH,
        (nc::LINES() - PLAYGROUND_HEIGHT) / 2,
        (nc::COLS() - PLAYGROUND_WIDTH) / 2,
    )
}

pub fn init_color_pairs() {
    nc::start_color();
    nc::init_pair(1, nc::COLOR_RED, nc::COLOR_RED);
}

pub fn draw_snake(screen: nc::WINDOW, snake: &Snake) {
    nc::mvwaddstr(screen, snake.head.y, snake.head.x * 2, "██");
    snake.body.iter().for_each(|piece| {
        nc::mvwaddstr(screen, piece.y, piece.x * 2, "██");
    });
}

pub fn draw_food(screen: nc::WINDOW, food: &Food) {
    nc::wattr_on(screen, nc::COLOR_PAIR(1));
    nc::mvwaddstr(screen, food.coord.y, food.coord.x * 2, "██");
    nc::wattroff(screen, nc::COLOR_PAIR(1));
}

pub fn draw_score(screen: nc::WINDOW, score: i32) {
    nc::mvwaddstr(
        screen,
        PLAYGROUND_HEIGHT - 1,
        2,
        &format!("SCORE: {}", score),
    );
}
