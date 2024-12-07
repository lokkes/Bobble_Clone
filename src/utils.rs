use crate::grid::{ GRID_WIDTH, GRID_HEIGHT, BLOCK_SIZE };

pub fn check_collision(grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT], x: f32, y: f32) -> bool {
    let grid_x = (x / BLOCK_SIZE) as usize;
    let grid_y = (y / BLOCK_SIZE) as usize;

    if grid_x < 32 && grid_y < 18 {
        grid[grid_y][grid_x]
    } else {
        false
    }
}
