use crate::grid::{ GRID_WIDTH, GRID_HEIGHT, BLOCK_SIZE };

pub fn check_collision(grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT], x: f32, y: f32) -> bool {
    let grid_x = (x / BLOCK_SIZE) as usize;
    let grid_y = (y / BLOCK_SIZE) as usize;

    if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
        grid[grid_y][grid_x]
    } else {
        false
    }
}

pub fn check_collision_player(grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT], x: f32, y: f32) -> bool {
    check_collision(grid, x, y) ||
        check_collision(grid, x + BLOCK_SIZE, y) ||
        check_collision(grid, x - BLOCK_SIZE, y)
}
