use macroquad::prelude::*;

mod control_panel;
mod debugger;
mod field;
mod game;
mod observer;
mod monitor;

use control_panel::ControlPanel;
use debugger::Debugger;
use game::{FieldType, Game};
use observer::Observer;

#[macroquad::main("MyGame")]
async fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();

    let field_type = if args.contains(&"--dims=1".to_string()) {
        FieldType::Field1D
    } else {
        FieldType::Field2D
    };

    let mut game = Game::new(field_type);
    let mut observers: Vec<Box<dyn Observer>> =
        vec![Box::new(Debugger::new()), Box::new(ControlPanel::new())];
    

    
    loop {
        clear_background(WHITE);

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
