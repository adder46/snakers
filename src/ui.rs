use crate::core::{Food, Snake};
use ncurses::*;

pub const PLAYGROUND_HEIGHT: i32 = 24;
pub const PLAYGROUND_WIDTH: i32 = 80;

pub fn draw_snake(screen: WINDOW, snake: &Snake) {
    for piece in snake.body().iter() {
        mvwaddstr(screen, piece.y(), piece.x() * 2, "██");
    }
}

pub fn draw_food(screen: WINDOW, food: &Food) {
    wattr_on(screen, COLOR_PAIR(1));
    mvwaddstr(screen, food.y(), food.x() * 2, "██");
    wattroff(screen, COLOR_PAIR(1));
}

pub fn create_playground() -> WINDOW {
    let inner_screen = newwin(
        PLAYGROUND_HEIGHT,
        PLAYGROUND_WIDTH,
        (LINES() - PLAYGROUND_HEIGHT) / 2,
        (COLS() - PLAYGROUND_WIDTH) / 2,
    );
    inner_screen
}

pub fn init_color_pairs() {
    start_color();
    init_pair(1, COLOR_RED, COLOR_RED);
}
