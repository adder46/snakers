use rand::Rng;
use std::collections::VecDeque;

use crate::ui::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH};

pub struct Game {
    pub food: Food,
    pub snake: Snake,
    pub food_counter: i32,
    pub score: i32,
    pub paused: bool,
}

impl Game {
    pub fn new() -> Game {
        let snake = Snake::new();
        Game {
            food: Food::new(&snake),
            snake,
            food_counter: 0,
            score: 0,
            paused: false,
        }
    }

    pub fn init_snake(&mut self) {
        for i in -3..4 {
            self.snake.body.push_front(Coord::new(
                PLAYGROUND_HEIGHT / 2,
                (PLAYGROUND_WIDTH / 4) + i,
            ));
        }
    }

    pub fn handle_food(&mut self) {
        self.food_counter += 1;
        if self.food_counter == 100 {
            self.make_new_food();
        }
        if self.snake.is_touching_food(&self.food) {
            self.snake.eat_food(&self.food);
            self.score += 1;
            self.make_new_food();
        }
    }

    pub fn snake_about_to_collide(&self, next_step: &Coord) -> bool {
        self.snake.body.contains(next_step)
            || [0, PLAYGROUND_HEIGHT].contains(&self.snake.body.front().unwrap().y)
            || [0, PLAYGROUND_WIDTH / 2].contains(&self.snake.body.front().unwrap().x)
    }

    pub fn get_next_step(&self) -> Coord {
        let Coord { y, x } = self.snake.body.front().unwrap();
        let (next_y, next_x) = match self.snake.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        Coord::new(y + next_y, x + next_x)
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new();
        self.init_snake();
        self.food = Food::new(&self.snake);
        self.food_counter = 0;
        self.score = 0;
        self.paused = false;
    }

    fn make_new_food(&mut self) {
        self.food_counter = 0;
        self.food = Food::new(&self.snake);
    }
}

pub struct Snake {
    pub body: VecDeque<Coord>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: VecDeque::new(),
            direction: Direction::Right,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.forbidden_direction(&self.direction) {
            self.direction = direction;
        }
    }

    pub fn crawl(&mut self, next_step: &Coord, paused: bool) {
        if !paused {
            self.body.push_front(*next_step);
            self.body.pop_back();
        }
    }

    pub fn is_touching_food(&self, food: &Food) -> bool {
        *self.body.front().unwrap() == food.coord
    }

    pub fn eat_food(&mut self, food: &Food) {
        self.body.push_front(food.coord);
    }

    fn forbidden_direction(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq)]
pub struct Food {
    pub coord: Coord,
}

impl Food {
    fn new(snake: &Snake) -> Food {
        let mut rng = rand::thread_rng();
        loop {
            let y = rng.gen_range(1, PLAYGROUND_HEIGHT - 1);
            let x = rng.gen_range(1, (PLAYGROUND_WIDTH / 2) - 1);
            if snake.body.contains(&Coord::new(y, x)) {
                continue;
            } else {
                return Food {
                    coord: Coord::new(y, x),
                };
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Coord {
    pub y: i32,
    pub x: i32,
}

impl Coord {
    pub fn new(y: i32, x: i32) -> Coord {
        Coord { y, x }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
