use ggez::graphics::DrawParam;
use crate::game::Game;

pub const GRID_WIDTH: usize = 32;
pub const GRID_HEIGHT: usize = 18;
pub const BLOCK_SIZE: f32 = 25.0;

pub fn create_grid() -> [[bool; GRID_WIDTH]; GRID_HEIGHT] {
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];

    // left vertical line
    for y in 0..GRID_HEIGHT {
        for x in 0..2 {
            grid[y][x] = true;
        }
    }

    // Right vertical line
    for y in 0..GRID_HEIGHT {
        for x in GRID_WIDTH - 2..GRID_WIDTH {
            grid[y][x] = true;
        }
    }

    // top grid
    for x in 1..8 {
        grid[0][x] = true; // Bottom platform
    }

    for x in 12..20 {
        grid[0][x] = true;
    }

    for x in 24..32 {
        grid[0][x] = true;
    }

    // middle grid
    for x in 5..27 {
        grid[9][x] = true; // Middle platform
    }

    // bottom grid (3 grids needed)
    for x in 1..8 {
        grid[17][x] = true; // Bottom platform
    }

    for x in 12..20 {
        grid[17][x] = true;
    }

    for x in 24..32 {
        grid[17][x] = true;
    }

    //bottom middle blocks
    for x in 1..11 {
        grid[13][x] = true;
    }

    for x in 21..32 {
        grid[13][x] = true;
    }

    // Floating blocks in the center
    for x in 5..10 {
        grid[5][x] = true; // Small block above middle platform
    }

    for x in 22..27 {
        grid[5][x] = true; // Another small block
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
