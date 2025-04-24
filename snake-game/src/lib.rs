use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, window};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

struct InnerGame {
    snake: Vec<(i32, i32)>,
    direction: (i32, i32),
    food: (i32, i32),
    width: i32,
    height: i32,
    interval: i32,
    score: i32,
    direction_changed: bool,
    context: CanvasRenderingContext2d,
    key_event_closure: Option<Closure<dyn FnMut(KeyboardEvent)>>,
    interval_closure: Option<Closure<dyn FnMut()>>,
    interval_id: Option<i32>,
}

#[derive(Clone)]
pub struct Game {
    inner: Rc<RefCell<InnerGame>>,
}

const ELEMENT_SIZE: i32 = 10;

thread_local! {
    static SCORE_CALLBACK: RefCell<Option<js_sys::Function>> = RefCell::new(None);
    static GAME_OVER_CALLBACK: RefCell<Option<js_sys::Function>> = RefCell::new(None);
}

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
                food: (0, 0),
                width,
                height,
                interval: 100,
                score: 0,
                direction_changed: false,
                context,
                key_event_closure: None,
                interval_closure: None,
                interval_id: None,
            })),
        };
        
        // Set up the keyboard event listener
        game.setup_keyboard_listener();
        game.spawn_food();
        
        game
    }

    fn setup_keyboard_listener(&mut self) {
        let window = window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");
        
        // Add the event listener to the document
        let mut game_clone = self.clone();
        let closure = Closure::new(move |event: KeyboardEvent| {            
            match event.key().as_str() {
                "ArrowUp" | "w" => game_clone.update_direction((0, -1)),
                "ArrowDown" | "s" => game_clone.update_direction((0, 1)),
                "ArrowLeft" | "a" => game_clone.update_direction((-1, 0)),
                "ArrowRight" | "d" => game_clone.update_direction((1, 0)),
                _ => (),
            }
        });
        
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("could not add event listener");
            
        // Store the closure to prevent it from being dropped
        let mut inner = self.inner.borrow_mut();
        inner.key_event_closure = Some(closure);
    }

    fn update_direction(&mut self, direction: (i32, i32)) {
        let mut inner = self.inner.borrow_mut();
        if inner.direction_changed {
            return; // Only one direction change per loop
        }
        if direction == (-inner.direction.0, -inner.direction.1) {
            return; // Don't reverse the snake's direction
        }
        inner.direction = direction;
        inner.direction_changed = true;
    }

    fn update(&mut self) {
        // Reset direction_changed at the start of each loop
        self.inner.borrow_mut().direction_changed = false;
        let (ate_food, game_over) = self.update_logic();

        if game_over {
            self.stop();

            GAME_OVER_CALLBACK.with(|f| {
                if let Some(cb) = f.borrow().as_ref() {
                    let _ = cb.call0(&JsValue::NULL);
                }
            });
        }
        
        if ate_food {
            self.inner.borrow_mut().score += 1;
            self.spawn_food();
            self.speed_up();

            SCORE_CALLBACK.with(|f| {
                if let Some(cb) = f.borrow().as_ref() {
                    let _ = cb.call1(&JsValue::NULL, &JsValue::from(self.inner.borrow().score));
                }
            });
        }
    }

    fn update_logic(&mut self) -> (bool, bool) {
        let mut inner = self.inner.borrow_mut();

        if inner.direction == (0, 0) {
            return (false, false);
        }

        let head = inner.snake.last().unwrap();
        let new_head = (
            (head.0 + inner.direction.0 + inner.width) % inner.width,
            (head.1 + inner.direction.1 + inner.height) % inner.height,
        );

        if inner.snake.contains(&new_head) {
            return (false, true);
        }

        inner.snake.push(new_head);
        if new_head == inner.food {
            (true, false)
        } else {
            inner.snake.remove(0);
            (false, false)
        }
    }

    fn spawn_food(&mut self) {
        let mut inner = self.inner.borrow_mut();
        loop {
            let x = (getrandom::u32().unwrap() % inner.width as u32) as i32;
            let y = (getrandom::u32().unwrap() % inner.height as u32) as i32;
            if !inner.snake.contains(&(x, y)) {
                inner.food = (x, y);
                break;
            }
        }
    }

    fn draw(&self) {
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

    fn speed_up(&mut self) {
        let window = web_sys::window().unwrap();
        let mut inner = self.inner.borrow_mut();

        // Clear the old interval if it exists
        if let Some(id) = inner.interval_id {
            window.clear_interval_with_handle(id);
        }

        inner.interval = (inner.interval as f64 * 0.99) as i32;

        if let Some(ref closure) = inner.interval_closure {
            let id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    inner.interval,
                )
                .unwrap();
            inner.interval_id = Some(id);
        }
    }

    fn stop(&mut self) {
        let inner = self.inner.borrow_mut();
        if let Some(id) = inner.interval_id {
            window().unwrap().clear_interval_with_handle(id);
        }
    }

    pub fn start(&mut self) {
        let mut game_clone = self.clone();
        let closure = Closure::wrap(Box::new(move || {
            game_clone.update();
            game_clone.draw();
        }) as Box<dyn FnMut()>);

        let window = window().unwrap();
        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            self.inner.borrow().interval,
        ).unwrap();

        // Store the closure and interval id to prevent them from being dropped
        let mut inner = self.inner.borrow_mut();
        inner.interval_closure = Some(closure);
        inner.interval_id = Some(id);
    }
}

#[wasm_bindgen]
pub struct GameWrapper {
    game: Option<Game>,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl GameWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> GameWrapper {
        GameWrapper {
            game: None,
            canvas,
        }
    }

    pub fn start(&mut self, score_callback: JsValue, game_over_callback: JsValue) {
        SCORE_CALLBACK.with(|f| {
            *f.borrow_mut() = Some(score_callback.dyn_into::<js_sys::Function>().unwrap());
        });
        GAME_OVER_CALLBACK.with(|f| {
            *f.borrow_mut() = Some(game_over_callback.dyn_into::<js_sys::Function>().unwrap());
        });

        let mut game = Game::new(&self.canvas);

        game.start();
        self.game = Some(game);
    }

    pub fn restart(&mut self, score_callback: JsValue, game_over_callback: JsValue) {
        self.start(score_callback, game_over_callback);
    }
}
