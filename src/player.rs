use crate::grid::{GRID_WIDTH, GRID_HEIGHT};

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
}
