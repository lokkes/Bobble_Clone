use crate::{ game::Game, grid::{ GRID_HEIGHT, GRID_WIDTH } };
use ggez::graphics::{ self, DrawParam };

pub struct Bullet {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
    pub image: graphics::Image,
}

impl Bullet {
    pub fn update(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
    }

    pub fn is_off_screen(&self, block_size: f32) -> bool {
        self.pos.0 < 0.0 ||
            self.pos.0 > (GRID_WIDTH as f32) * block_size ||
            self.pos.1 < 0.0 ||
            self.pos.1 > (GRID_HEIGHT as f32) * block_size
    }

    pub fn draw(canvas: &mut ggez::graphics::Canvas, game: &mut Game) {
        for bullet in game.bullets.iter() {
            canvas.draw(
                &bullet.image,
                DrawParam::default()
                    .dest(ggez::mint::Point2 {
                        x: bullet.pos.0,
                        y: bullet.pos.1 - game.block_size / 2.0,
                    })
                    .scale(ggez::mint::Vector2 {
                        x: (game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285) *
                        0.5,
                        y: (game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285) *
                        0.5,
                    })
            );
        }
    }
}
