pub fn check_collision(x: f32, y: f32, grid: &[[bool; 32]; 18]) -> bool {
    let grid_x = (x / 25.0) as usize;
    let grid_y = (y / 25.0) as usize;
    grid[grid_y][grid_x]
}
