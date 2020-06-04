use std::collections::VecDeque;
use rand::Rng;

pub struct Game {
    screen_height: i32,
    screen_width: i32,
    food_counter: i32,
    food: Food,
    snake: Snake
}

impl Game {
    pub fn new(screen_height: i32, screen_width: i32) -> Game {
        Game { 
            screen_height,
            screen_width,
            food_counter: 0,
            food: Food::new(screen_height, screen_width),
            snake: Snake::new()
        }
    }

    pub fn handle_food(&mut self) {
        self.food_counter += 1;
        if self.food_counter == 100 {
            self.food_counter = 0;
            self.food = Food::new(self.screen_height, self.screen_width);
            self.food.position_properly(&self.snake, self.screen_height, self.screen_width);
        }
        if self.snake.is_touching_food(&self.food) {
            self.snake.eat_food(&self.food);
            self.food_counter = 0;
            self.food = Food::new(self.screen_height, self.screen_width);
            self.food.position_properly(&self.snake, self.screen_height, self.screen_width);
        }
    }

    pub fn snake_about_to_collide(&self, next_step: &Coord) -> bool {
        self.snake.body.contains(next_step) || 
        [0, self.screen_height].contains(&self.snake.body.front().unwrap().y()) || 
        [0, self.screen_width].contains(&self.snake.body.front().unwrap().x())
    }

    pub fn get_next_step(&self) -> Coord {
        let (y, x) = match self.snake.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        };
        let next_step = Coord::new(
            self.snake.body.front().unwrap().y + y,
            self.snake.body.front().unwrap().x + x
        );
        next_step
    }

    pub fn snake(&mut self) -> &mut Snake {
        &mut self.snake
    }

    pub fn food(&mut self) -> &mut Food {
        &mut self.food
    }
}

pub struct Snake {
    body: VecDeque<Coord>,
    direction: Direction
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: VecDeque::new(),
            direction: Direction::Right
        }
    }

    pub fn init_body(&mut self, y: i32, x: i32) {
        for i in -3..4 {
            self.body.push_front(
                Coord::new(y, x + i)
            );
        }
    }

    pub fn body(&self) -> &VecDeque<Coord> {
        &self.body
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.forbidden_direction(&self.direction) {
            self.direction = direction;
        }
    }

    pub fn crawl(&mut self, next_step: &Coord) {
        self.body.push_front(*next_step);
        self.body.pop_back();
    }

    pub fn is_touching_food(&self, food: &Food) -> bool {
        self.body.front().unwrap().y() == food.y() &&
        self.body.front().unwrap().x() == food.x()
    }

    pub fn eat_food(&mut self, food: &Food) {
        self.body.push_front(
            Coord::new(food.y(), food.x())
        );
    }

    fn forbidden_direction(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

pub struct Food {
    x: i32,
    y: i32
}

impl Food {
    fn new(screen_height: i32, screen_width: i32) -> Food {
        let mut rng = rand::thread_rng();
        Food { 
            y: rng.gen_range(1, screen_height - 1),
            x: rng.gen_range(1, screen_width - 1)
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn position_properly(&mut self, snake: &Snake, screen_height: i32, screen_width: i32) {
        loop {
            if snake.body.contains(&Coord::new(self.y, self.x)) {
                let mut rng = rand::thread_rng();
                self.y = rng.gen_range(1, screen_height - 1);
                self.x = rng.gen_range(1, screen_width - 1);
            } else { break; }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    pub fn new(y: i32, x: i32) -> Coord {
        Coord { y, x }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }   
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
