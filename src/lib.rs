use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = Vec::new();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }

        Self {
            body,
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    reward_cell: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        let size: usize = width * width;
        let snake = Snake::new(snake_index, 3);

        Self {
            width,
            size: width * width,
            reward_cell: World::gen_reward_cell(size),
            snake,
        }
    }

    // TODO: 蛋不能在蛇身上
    fn gen_reward_cell(max: usize) -> usize {
        random(max)
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn update(&mut self) {
        let snake_head_index = self.snake_head_index();
        let (row, col) = self.index_to_cell(snake_head_index);

        let (row, col) = match self.snake.direction {
            Direction::Left => (row, (col - 1) % self.width),
            Direction::Right => (row, (col + 1) % self.width),
            Direction::Up => ((row - 1) % self.width, col),
            Direction::Down => ((row + 1) % self.width, col),
        };

        let next_index = self.cell_to_index(row, col);
        self.set_snake_head(next_index);
    }

    fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }

    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}

