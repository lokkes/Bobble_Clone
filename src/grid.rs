pub const GRID_WIDTH: usize = 32;
pub const GRID_HEIGHT: usize = 18;
pub const BLOCK_SIZE: f32 = 25.0; 

pub fn create_grid() -> [[bool; GRID_WIDTH]; GRID_HEIGHT] {
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
    // Example layout
    for x in 5..10 {
        grid[16][x] = true; 
    }
    for y in 3..8 {
        grid[y][15] = true;
    }
    grid
}
