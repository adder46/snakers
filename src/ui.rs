use ncurses::*;
use crate::core::{Food, Snake};


const MIN_HEIGHT: i32 = 24;
const MIN_WIDTH: i32 = 80;

pub fn draw_snake(snake: &Snake, screen: WINDOW) {
    for piece in snake.body().iter() {
        mvwaddstr(screen, piece.y(), piece.x() * 2, "██");
    }
}

pub fn draw_food(food: &Food, screen: WINDOW) {
    mvwaddstr(screen, food.y(), food.x() * 2, "██");
}

pub fn create_playground() -> WINDOW {
    let inner_screen = newwin(
        MIN_HEIGHT, MIN_WIDTH,
        (LINES() / 2) - (MIN_HEIGHT / 2),
        (COLS() / 2) - (MIN_WIDTH / 2)
    );
    inner_screen
}