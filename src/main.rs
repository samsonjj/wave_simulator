use macroquad::prelude::*;

mod control_panel;
mod debugger;
mod field;
mod game;
mod observer;

use control_panel::ControlPanel;
use debugger::Debugger;
use game::Game;
use observer::Observer;

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new();
    let mut observers: Vec<Box<dyn Observer>> =
        vec![Box::new(Debugger::new(true)), Box::new(ControlPanel::new())];
    loop {
        clear_background(WHITE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        game.update();
        for observer in observers.iter_mut() {
            observer.update(&game);
        }
        game.render();
        for observer in observers.iter() {
            observer.render(&game);
        }

        next_frame().await
    }
}
