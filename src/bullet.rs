use crate::grid::{GRID_WIDTH, GRID_HEIGHT, BLOCK_SIZE};

pub struct Bullet {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
}

impl Bullet {
    pub fn update(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
    }

    pub fn is_off_screen(&self) -> bool {
        self.pos.0 < 0.0 || self.pos.0 > GRID_WIDTH as f32 * BLOCK_SIZE || self.pos.1 < 0.0 || self.pos.1 > GRID_HEIGHT as f32 * BLOCK_SIZE
    }
}
