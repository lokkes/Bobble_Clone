use crate::grid::{GRID_WIDTH, GRID_HEIGHT};

pub struct Enemy {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
}

impl Enemy {
    pub fn update(&mut self, grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;

        if self.pos.0 < 0.0 || self.pos.0 > GRID_WIDTH as f32 * 25.0 - 25.0 {
            self.velocity.0 = -self.velocity.0;
        }

        if self.pos.1 < 0.0 || self.pos.1 > GRID_HEIGHT as f32 * 25.0 - 25.0 {
            self.velocity.1 = -self.velocity.1;
        }

        // Handle collision with grid obstacles
        let grid_x = (self.pos.0 / 25.0) as usize;
        let grid_y = (self.pos.1 / 25.0) as usize;
        if grid[grid_y][grid_x] {
            self.velocity = (0.0, 0.0); // Stop moving if it hits an obstacle
        }
    }
}
