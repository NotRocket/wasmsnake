mod gameboard;
use gameboard::*;

use std::time::{Duration};
use wasm_bindgen::prelude::*;
use web_sys::window;
use speedy2d::{
    Graphics2D,
    WebCanvas,
};
use speedy2d::color::Color;
use speedy2d::window::{
    KeyScancode,
    VirtualKeyCode,
    WindowHandler,
    WindowHelper,
    WindowStartupInfo,
};

fn now() -> f64 {
    window()
        .expect("no global `window`")
        .performance()
        .expect("Performance should be available")
        .now()
}

struct MyWindowHandler {
    last_key: Option<VirtualKeyCode>,
    last_time: f64,
    game_board: Option<GameBoard>,
    snake: Option<Snake>,
}

impl MyWindowHandler{
    fn new() -> Self{
        let mut wh = MyWindowHandler{last_key: None, last_time: now(),game_board: None,snake: None};
        wh.game_board = Option::from(GameBoard::new(25, 25));
        wh.snake = Option::from(Snake::new(wh.game_board.as_mut().unwrap()));
        wh
    }
    fn reset(&mut self){
        self.game_board = Option::from(GameBoard::new(25, 25));
        self.snake = Option::from(Snake::new(self.game_board.as_mut().unwrap()));
    }
}

impl WindowHandler for MyWindowHandler
{
    fn on_start(&mut self, helper: &mut WindowHelper, _info: WindowStartupInfo){
        helper.set_title("Rusty Snake");
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        let mut x: f32 = 50.0;
        let mut y: f32 = 50.0;
        for tileVector in self.game_board.as_mut().unwrap().board.iter_mut() {
            for tile in tileVector.iter_mut() {
                graphics.draw_rectangle(
                    speedy2d::shape::Rectangle::from_tuples((x, y), (x + 49.0, y + 49.0)),
                    match tile.state {
                        TileState::FoodOccupied => { Color::RED }
                        TileState::BorderDeathZone => { Color::BLACK }
                        TileState::Free => { Color::WHITE }
                        TileState::SnakeOccupied => { if self.snake.as_ref().unwrap().alive {
                            Color::BLUE
                        }else{
                            Color::GREEN
                        }
                        }
                    },
                );
                x += 50.0;
            }
            x = 50.0;
            y += 50.0;
        }
        let now_ms = now();
        if self.snake.as_ref().unwrap().alive{
            if now_ms - self.last_time > 250.0 {
                match self.last_key {
                    Some(key) => {self.snake.as_mut().unwrap().move_snake(self.game_board.as_mut().unwrap(), key); },
                    None => { log::info!("No key pressed") }
                }

                self.last_time = now_ms;
            }
        }else if !self.snake.as_ref().unwrap().alive{
            match self.last_key {
                Some(key) =>{if key == VirtualKeyCode::Space{
                    self.reset();
                    self.last_key = None;
                }}
                None => {}
            }
        }

        helper.request_redraw();
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        if let Some(key) = virtual_key_code {
            self.last_key = Some(key);
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
compile_error!("This sample only builds for WebAssembly (wasm32)");

#[wasm_bindgen]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting Rusty Snake!");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    WebCanvas::new_for_id("game-canvas", MyWindowHandler::new()).unwrap();
}
