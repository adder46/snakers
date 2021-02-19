use rand::Rng;
use std::collections::VecDeque;
use std::mem;
use std::ops::Add;

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
        let snake = Snake::from_coords(
            (-3..4).map(|i| Coord::new(PLAYGROUND_HEIGHT / 2, (PLAYGROUND_WIDTH / 4) + i)),
        );
        Game {
            food: Food::new(&snake),
            snake,
            food_counter: 0,
            score: 0,
            paused: false,
        }
    }

    pub fn handle_food(&mut self) {
        self.food_counter += 1;
        if self.food_counter == 100 {
            self.make_new_food();
        }
        if self.snake.is_touching_food(self.food) {
            self.snake.eat_food(self.food);
            self.score += 1;
            self.make_new_food();
        }
    }

    pub fn snake_about_to_collide(&self, next_step: Coord) -> bool {
        self.snake.body.contains(&next_step)
            || [0, PLAYGROUND_HEIGHT - 1].contains(&self.snake.head.y)
            || [0, PLAYGROUND_WIDTH / 2].contains(&self.snake.head.x)
    }

    pub fn get_next_step(&self) -> Coord {
        let next_step = match self.snake.direction {
            Direction::Up => Coord::new(-1, 0),
            Direction::Down => Coord::new(1, 0),
            Direction::Left => Coord::new(0, -1),
            Direction::Right => Coord::new(0, 1),
        };
        self.snake.head + next_step
    }

    pub fn restart(&mut self) {
        *self = Self::new()
    }

    fn make_new_food(&mut self) {
        self.food_counter = 0;
        self.food = Food::new(&self.snake);
    }
}

#[derive(Clone)]
pub struct Snake {
    pub head: Coord,
    pub body: VecDeque<Coord>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            head: Coord::new(0, 0),
            body: VecDeque::new(),
            direction: Direction::Right,
        }
    }

    fn from_coords(mut coords: impl DoubleEndedIterator<Item = Coord>) -> Snake {
        let mut snake = Self::new();
        if let Some(coord) = coords.next_back() {
            snake.head = coord;
        }
        coords.for_each(|coord| {
            snake.body.push_front(coord);
        });
        snake
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.forbidden_direction(self.direction) {
            self.direction = direction;
        }
    }

    pub fn crawl(&mut self, next_step: Coord, paused: bool) {
        if !paused {
            let old_head = mem::replace(&mut self.head, next_step);
            self.body.push_front(old_head);
            self.body.pop_back();
        }
    }

    pub fn is_touching_food(&self, food: Food) -> bool {
        self.head == food.coord
    }

    pub fn eat_food(&mut self, food: Food) {
        let old_head = mem::replace(&mut self.head, food.coord);
        self.body.push_front(old_head);
    }

    fn forbidden_direction(&self, direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub y: i32,
    pub x: i32,
}

impl Coord {
    pub fn new(y: i32, x: i32) -> Coord {
        Coord { y, x }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        direction, next_step,
        case(Direction::Up, Coord::new(-1, 0)),
        case(Direction::Down, Coord::new(1, 0)),
        case(Direction::Left, Coord::new(0, 1)),
        case(Direction::Right, Coord::new(0, 1)),
    )]
    fn get_next_step(direction: Direction, next_step: Coord) {
        let mut game = Game::new();
        game.snake.set_direction(direction);
        assert_eq!(game.get_next_step(), game.snake.head + next_step);
    }

    #[rstest(
        current,
        next,
        expected,
        case(Direction::Up, Direction::Left, Direction::Left),
        case(Direction::Up, Direction::Right, Direction::Right),
        case(Direction::Up, Direction::Up, Direction::Up),
        case(Direction::Up, Direction::Down, Direction::Up),
        case(Direction::Down, Direction::Left, Direction::Left),
        case(Direction::Down, Direction::Right, Direction::Right),
        case(Direction::Down, Direction::Down, Direction::Down),
        case(Direction::Down, Direction::Up, Direction::Down),
        case(Direction::Left, Direction::Up, Direction::Up),
        case(Direction::Left, Direction::Down, Direction::Down),
        case(Direction::Left, Direction::Left, Direction::Left),
        case(Direction::Left, Direction::Right, Direction::Left),
        case(Direction::Right, Direction::Up, Direction::Up),
        case(Direction::Right, Direction::Down, Direction::Down),
        case(Direction::Right, Direction::Right, Direction::Right),
        case(Direction::Right, Direction::Left, Direction::Right)
    )]
    fn set_direction(current: Direction, next: Direction, expected: Direction) {
        let mut snake = Snake::new();
        snake.direction = current;
        snake.set_direction(next);
        assert_eq!(snake.direction, expected);
    }

    #[rstest(coord, case(Coord::new(1, 5)), case(Coord::new(9, 3)))]
    fn eat_food(coord: Coord) {
        let mut snake = Snake::new();
        let mut food = Food::new(&snake);
        food.coord = coord;
        snake.eat_food(food);
        assert_eq!(snake.head, food.coord);
    }

    #[rstest()]
    fn is_touching_food() {
        let snake = Snake::new();
        let mut food = Food::new(&snake);
        food.coord = snake.head;
        assert!(snake.is_touching_food(food));
    }

    #[rstest()]
    fn crawl() {
        let mut game = Game::new();
        let next_step = game.get_next_step();
        let old_snake = game.snake.clone();
        game.snake.crawl(next_step, false);
        assert_eq!(game.snake.head, next_step);
        assert_eq!(game.snake.body.front(), Some(&old_snake.head));
        assert!(!game.snake.body.contains(old_snake.body.back().unwrap()));
    }

    #[rstest(
        current,
        forbidden,
        case(Direction::Up, Direction::Down),
        case(Direction::Down, Direction::Up),
        case(Direction::Left, Direction::Right),
        case(Direction::Right, Direction::Left)
    )]
    fn forbidden_direction(current: Direction, forbidden: Direction) {
        let snake = Snake::new();
        assert_eq!(snake.forbidden_direction(current), forbidden);
    }
}
