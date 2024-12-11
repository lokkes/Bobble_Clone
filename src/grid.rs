use ggez::graphics::DrawParam;
use crate::game::Game;

pub const GRID_WIDTH: usize = 32;
pub const GRID_HEIGHT: usize = 18;
pub const BLOCK_SIZE: f32 = 25.0;

#[derive(Clone)]
pub struct GridConfig {
    pub vertical_lines: Vec<(usize, usize, usize)>, // (start_x, start_y, end_y)
    pub horizontal_lines: Vec<(usize, usize, usize)>, // (y,start_x, end_x)
}

pub fn create_grid(config: &GridConfig) -> [[bool; GRID_WIDTH]; GRID_HEIGHT] {
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];

    // Vertical lines
    for &(start_x, start_y, end_y) in &config.vertical_lines {
        for y in start_y..end_y {
            grid[y][start_x] = true;
        }
    }

    // Horizontal lines
    for &(y, start_x, end_x) in &config.horizontal_lines {
        for x in start_x..end_x {
            grid[y][x] = true;
        }
    }

    grid
}

pub fn draw(
    canvas: &mut ggez::graphics::Canvas,
    game: &mut Game
) -> Result<(), Box<dyn std::error::Error>> {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if game.grid[y][x] {
                canvas.draw(
                    &game.grid_image,
                    DrawParam::default().dest(ggez::mint::Point2 {
                        x: (x as f32) * BLOCK_SIZE,
                        y: (y as f32) * BLOCK_SIZE,
                    })
                );
            }
        }
    }
    Ok(())
}
