use ggez::{ ContextBuilder, GameResult};
use ggez::event::{self};
use ggez::conf::WindowMode;

mod game;
mod player;
mod enemy;
mod grid;
mod utils;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("bobble_clone", "author_name")
        .window_setup(ggez::conf::WindowSetup::default().title("bobble_clone"))
        .window_mode(WindowMode::default().dimensions(800.0, 480.0))
        .build()?;

    let game = game::Game::new();
    event::run(ctx, event_loop, game)
}
