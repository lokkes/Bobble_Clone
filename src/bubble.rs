use ggez::graphics::DrawParam;

use crate::{ game::Game, grid::GRID_WIDTH };

pub struct Bubble {
    pos: (f32, f32),
    velocity: (f32, f32),
    current_frame: usize,
    frame_timer: f32,
}

impl Bubble {
    pub fn new(pos: (f32, f32), velocity: (f32, f32)) -> Self {
        Bubble {
            pos,
            velocity,
            current_frame: 0,
            frame_timer: 0.0,
        }
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        self.pos.1 += self.velocity.1;

        let delta_time = ctx.time.delta().as_secs_f32();
        self.frame_timer += delta_time;
        if self.frame_timer >= 0.1 {
            self.current_frame = (self.current_frame + 1) % 8;
            self.frame_timer = 0.0;
        }
    }

    pub fn draw(canvas: &mut ggez::graphics::Canvas, game: &mut Game) {
        for bubble in &game.bubbles {
            let image = game.resources.bobble_image[bubble.current_frame].clone();
            for bubble in &game.bubbles {
                canvas.draw(
                    &image,
                    DrawParam::default()
                        .dest(ggez::mint::Point2 {
                            x: bubble.pos.0,
                            y: bubble.pos.1 - game.block_size,
                        })
                        .scale(ggez::mint::Vector2 {
                            x: game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285,
                            y: game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285,
                        })
                );
            }
        }
    }
}
