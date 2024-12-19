use crate::grid::{ GRID_WIDTH, GRID_HEIGHT };
use ggez::graphics;

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

}
