use ggez::graphics::Image;

use crate::grid::{ GRID_WIDTH, GRID_HEIGHT };

pub fn check_collision(
    grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT],
    x: f32,
    y: f32,
    block_size: f32
) -> bool {
    let grid_x = (x / block_size) as usize;
    let grid_y = (y / block_size) as usize;

    if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
        grid[grid_y][grid_x]
    } else {
        false
    }
}

pub fn check_collision_player(
    grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT],
    x: f32,
    y: f32,
    block_size: f32
) -> bool {
    check_collision(grid, x, y, block_size) ||
        check_collision(grid, x + block_size, y, block_size) ||
        check_collision(grid, x - block_size, y, block_size)
}

pub fn get_y_pos_correction(window_width: f32, block_size: f32, image: &Image) -> f32 {
    let y_pos_correction;
    if window_width >= 1920.0 {
        y_pos_correction =
            ((image.height() as f32) * block_size) / (GRID_WIDTH as f32) + block_size / 114.285;
    } else {
        y_pos_correction = block_size * 2.5;
    }
    y_pos_correction
}
