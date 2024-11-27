use crate::grid::{GRID_WIDTH, GRID_HEIGHT};
use ggez::graphics::DrawParam;
use crate::game::Game;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;

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

    pub fn draw( canvas: &mut ggez::graphics::Canvas, game: &mut Game, ctx:  &mut ggez::Context)-> Result<(), Box<dyn std::error::Error>>{
        let player_mesh=Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            ggez::mint::Point2 { x: 0.0, y: 0.0 },
            15.0,
            0.1,
            ggez::graphics::Color::from_rgb(255, 255, 255),
        )?;
        canvas.draw(
            &player_mesh,
            DrawParam::default().dest(ggez::mint::Point2 {
                x: game.player.pos.0 - 30.0,
                y: game.player.pos.1 - 50.0,
            }),
        );
        Ok(())
    }
   
}
