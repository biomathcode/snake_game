use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/view/utils/random.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    WON,
    LOST,
    PLAYED,
}

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }

        Snake {
            body,
            direction: Direction::RIGHT,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    points: usize,
    snake: Snake,
    size: usize,
    next_cell: Option<SnakeCell>, // Option => somevalue or none value
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        let snake = Snake::new(snake_idx, 3);

        let size = width * width;

        World {
            width,

            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
            points: 0,
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;

        loop {
            reward_cell = rnd(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        Some(reward_cell)
    }

    pub fn get_w(&self) -> usize {
        self.width
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn get_reward(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        // guard
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer
    // borrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::PLAYED)
    }
    pub fn end_game(&mut self) {
        self.status = None
    }

    pub fn get_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::WON) => String::from("You have won!"),
            Some(GameStatus::LOST) => String::from("You have lost!"),
            Some(GameStatus::PLAYED) => String::from("Playing"),
            None => String::from("No Status"),
        }
    }

    //  cannot return a reference to JS because of borrowing rules
    // pub fn snake_cells(&self) -> &Vec<SnakeCell> {
    //     &self.snake.body
    // }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::PLAYED) => {
                let temp = self.snake.body.clone();

                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    }
                    None => {
                        self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                    }
                }

                let len = self.snake.body.len();
                for i in 1..len {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0)
                }

                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::LOST)
                }

                if self.reward_cell == Some(self.snake_head()) {
                    if self.snake_length() < self.size {
                        self.points += 1;
                        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body)
                    } else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::WON)
                    }
                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head();

        let row = snake_idx / self.width;

        return match direction {
            Direction::RIGHT => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::LEFT => {
                let threshold = (row) * self.width;
                if snake_idx == threshold {
                    SnakeCell(threshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::UP => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::DOWN => {
                let threshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }
}

// use extern to get function in the webassembly
// wasm-pack build --target web
// modular is expensive and divide is expensive
