use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::conf::WindowMode;// for changing windowsize 



struct MainState;

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, ggez::graphics::Color::from_rgb(255, 255, 255));  // White color
        ggez::graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("bobble_clone", "author_name").window_mode(WindowMode::default().dimensions(2000.0, 1500.0));// length&width
    let (ctx, event_loop) = cb.build()?;
    let state = MainState;
    event::run(ctx, event_loop, state)
}



