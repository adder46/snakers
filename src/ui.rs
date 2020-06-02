use ncurses::*;
use crate::core::{Snake, Food};

pub fn draw_snake(snake: &Snake) {
    for piece in snake.body().iter() {
        mvaddstr(piece.y(), piece.x() * 2, "██");
    }
}

pub fn draw_food(food: &Food) {
    mvaddstr(food.y(), food.x() * 2, "██");
}