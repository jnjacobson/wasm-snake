use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console};

#[wasm_bindgen]
pub struct Game {
    snake: Vec<(i32, i32)>,
    direction: (i32, i32),
    food: (i32, i32),
    width: i32,
    height: i32,
    context: CanvasRenderingContext2d,
}

const ELEMENT_SIZE: i32 = 20;

#[wasm_bindgen]
impl Game {
    pub fn new(canvas: &HtmlCanvasElement) -> Game {
        let width = canvas.width() as i32 / ELEMENT_SIZE;
        let height = canvas.height() as i32 / ELEMENT_SIZE;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let snake = vec![(width / 2, height / 2)];
        Game {
            snake,
            direction: (0, 0),
            food: (3, 3), // Simple fixed food for now
            width,
            height,
            context,
        }
    }

    pub fn update(&mut self) {
        let head = self.snake.last().unwrap();
        let new_head = (
            (head.0 + self.direction.0 + self.width) % self.width,
            (head.1 + self.direction.1 + self.height) % self.height,
        );
        self.snake.push(new_head);
        
        if new_head != self.food {
            self.snake.remove(0);
        }
    }

    pub fn draw(&self) {
        // Clear canvas
        self.context.clear_rect(
            0.0,
            0.0,
            (self.width * ELEMENT_SIZE) as f64,
            (self.height * ELEMENT_SIZE) as f64,
        );

        // Draw snake
        let _ = self.context.set_fill_style_str("green");
        for &(x, y) in &self.snake {
            self.context.fill_rect(
                (x * ELEMENT_SIZE) as f64,
                (y * ELEMENT_SIZE) as f64,
                ELEMENT_SIZE as f64,
                ELEMENT_SIZE as f64,
            );
        }
        
        // Draw food
        let _ = self.context.set_fill_style_str("red");
        self.context.fill_rect(
            (self.food.0 * ELEMENT_SIZE) as f64,
            (self.food.1 * ELEMENT_SIZE) as f64,
            ELEMENT_SIZE as f64,
            ELEMENT_SIZE as f64,
        );
    }

    pub fn change_direction(&mut self, key: &str) {
        match key {
            "ArrowUp" => self.direction = (0, -1),
            "ArrowDown" => self.direction = (0, 1),
            "ArrowLeft" => self.direction = (-1, 0),
            "ArrowRight" => self.direction = (1, 0),
            " " => self.direction = (0, 0),
            _ => (),
        }
    }
}
