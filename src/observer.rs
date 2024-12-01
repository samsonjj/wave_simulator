use crate::game::Game;

pub trait Observer {
    fn render(&self, game: &Game);
    fn update(&mut self, game: &Game);
}
