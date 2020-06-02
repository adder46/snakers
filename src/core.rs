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
            food_counter: 0,
            food: Food::new(screen_height, screen_width),
            snake: Snake::new(),
            screen_height,
            screen_width
        }
    }

    pub fn handle_food(&mut self) {
        self.food_counter += 1;
        if self.food_counter == 100 {
            self.food_counter = 0;
            self.food = Food::new(self.screen_height, self.screen_width);
        }
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
                Coord { y, x: x + i }
            );
        }
    }

    pub fn body(&self) -> &VecDeque<Coord> {
        &self.body
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn crawl(&mut self) {
        let (y, x) = match self.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        };
        let next_step = Coord {
            y: self.body.front().unwrap().y + y,
            x: self.body.front().unwrap().x + x
        };
        self.body.push_front(next_step);
        self.body.pop_back();
    }
}

pub struct Food {
    x: i32,
    y: i32
}

impl Food {
    fn new(y: i32, x: i32) -> Food {
        let mut rng = rand::thread_rng();
        Food { 
            y: rng.gen_range(0, y),
            x: rng.gen_range(0, x)
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

pub struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }   
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
