use ggez::graphics::DrawParam;

use crate::{ game::Game, grid::GRID_WIDTH };

pub struct EnemyBullet {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
}

impl EnemyBullet {
    pub fn update(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
    }

    pub fn is_off_screen(&self) -> bool {
        self.pos.0 < 0.0 || self.pos.0 > 800.0 || self.pos.1 < 0.0 || self.pos.1 > 480.0
    }

    pub fn draw(canvas: &mut ggez::graphics::Canvas, game: &mut Game) {
        for bullet in &game.enemy_bullets {
            let image = game.resources.enemy_bullet_image[0].clone();
            canvas.draw(
                &image,
                DrawParam::default()
                    .dest(ggez::mint::Point2 {
                        x: bullet.pos.0,
                        y: bullet.pos.1 - game.block_size,
                    })
                    .scale(ggez::mint::Vector2 {
                        x: (game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285) /
                        2.0,
                        y: (game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285) /
                        2.0,
                    })
            );
        }
    }
}
