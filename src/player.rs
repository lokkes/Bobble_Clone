use crate::grid::{GRID_WIDTH, GRID_HEIGHT};
use ggez::graphics::DrawParam;
use crate::game::Game;

pub struct Player {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: (x, y),
            velocity: (0.0, 0.0),
        }
    }

    pub fn update_position(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;

        if self.pos.0 < 0.0 {
            self.pos.0 = 0.0;
        }
        if self.pos.0 > GRID_WIDTH as f32 * 25.0 - 25.0 {
            self.pos.0 = GRID_WIDTH as f32 * 25.0 - 25.0;
        }
    }

    pub fn draw( canvas: &mut ggez::graphics::Canvas, game: &mut Game){
        canvas.draw(
            &game.player_image,
            DrawParam::default().dest(ggez::mint::Point2 {
                x: game.player.pos.0 - 30.0,
                y: game.player.pos.1 - 50.0,
            }),
        );
    }
}
