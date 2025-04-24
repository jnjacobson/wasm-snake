use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console, KeyboardEvent, window};
use std::rc::Rc;
use std::cell::RefCell;

struct InnerGame {
    snake: Vec<(i32, i32)>,
    direction: (i32, i32),
    food: (i32, i32),
    width: i32,
    height: i32,
    context: CanvasRenderingContext2d,
    key_event_closure: Option<Closure<dyn FnMut(KeyboardEvent)>>,
}

#[derive(Clone)]
pub struct Game {
    inner: Rc<RefCell<InnerGame>>,
}

const ELEMENT_SIZE: i32 = 20;

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
        
        let mut game = Game {
            inner: Rc::new(RefCell::new(InnerGame {
                snake,
                direction: (0, 0),
                food: (3, 3), // Simple fixed food for now
                width,
                height,
                context,
                key_event_closure: None,
            })),
        };
        
        // Set up the keyboard event listener
        game.setup_keyboard_listener();
        
        game
    }

    fn setup_keyboard_listener(&mut self) {
        let window = window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");
        
        // Add the event listener to the document
        let game_clone = self.clone();
        let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
            let mut inner = game_clone.inner.borrow_mut();
            
            match event.key().as_str() {
                "ArrowUp" => inner.direction = (0, -1),
                "ArrowDown" => inner.direction = (0, 1),
                "ArrowLeft" => inner.direction = (-1, 0),
                "ArrowRight" => inner.direction = (1, 0),
                _ => (),
            }
        });
        
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("could not add event listener");
            
        // Store the closure to prevent it from being dropped
        let mut inner = self.inner.borrow_mut();
        inner.key_event_closure = Some(closure);
    }


    pub fn update(&mut self) {
        let mut inner = self.inner.borrow_mut();
        
        let head = inner.snake.last().unwrap();
        let new_head = (
            (head.0 + inner.direction.0 + inner.width) % inner.width,
            (head.1 + inner.direction.1 + inner.height) % inner.height,
        );
        inner.snake.push(new_head);
        
        if new_head != inner.food {
            inner.snake.remove(0);
        }
    }

    pub fn draw(&self) {
        let inner = self.inner.borrow_mut();

        inner.context.clear_rect(
            0.0,
            0.0,
            (inner.width * ELEMENT_SIZE) as f64,
            (inner.height * ELEMENT_SIZE) as f64,
        );

        // Draw snake
        let _ = inner.context.set_fill_style_str("green");
        for &(x, y) in &inner.snake {
            inner.context.fill_rect(
                (x * ELEMENT_SIZE) as f64,
                (y * ELEMENT_SIZE) as f64,
                ELEMENT_SIZE as f64,
                ELEMENT_SIZE as f64,
            );
        }
        
        // Draw food
        let _ = inner.context.set_fill_style_str("red");
        inner.context.fill_rect(
            (inner.food.0 * ELEMENT_SIZE) as f64,
            (inner.food.1 * ELEMENT_SIZE) as f64,
            ELEMENT_SIZE as f64,
            ELEMENT_SIZE as f64,
        );
    }
}

// Add a separate WASM-exported wrapper
#[wasm_bindgen]
pub struct SnakeGame(Game);

#[wasm_bindgen]
impl SnakeGame {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> SnakeGame {
        SnakeGame(Game::new(canvas))
    }

    pub fn update(&mut self) {
        self.0.update();
    }

    pub fn draw(&self) {
        self.0.draw();
    }
}
