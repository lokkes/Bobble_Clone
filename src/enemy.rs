use crate::grid::{GRID_WIDTH, GRID_HEIGHT, BLOCK_SIZE};
use crate::utils::check_collision;

pub struct Enemy {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
}

pub fn create_enemies() -> Vec<Enemy> {
    vec![
        Enemy { pos: (100.0, 100.0), velocity: (0.5, 0.0) },
        Enemy { pos: (300.0, 200.0), velocity: (0.0, 1.0) },
        Enemy { pos: (500.0, 200.0), velocity: (-1.0, -1.0) },
    ]
}

impl Enemy {
    pub fn update(&mut self, grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {

        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;

        if self.pos.0 < 0.0 || self.pos.0 > GRID_WIDTH as f32 * BLOCK_SIZE {
            self.velocity.0 = -self.velocity.0;
        }

        if self.pos.1 < 0.0 || self.pos.1 > GRID_HEIGHT as f32 * BLOCK_SIZE {
            self.velocity.1 = -self.velocity.1;
        }

       if check_collision(grid, self.pos.0, self.pos.1) {
            self.velocity.0 = -self.velocity.0; // Richtung umkehren
        }
      
    }
}
