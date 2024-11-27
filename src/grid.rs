use ggez::graphics::{ DrawMode,  Mesh, DrawParam};
use crate::game::Game;

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

pub fn draw(canvas: &mut ggez::graphics::Canvas,game: &mut Game, ctx:  &mut ggez::Context)-> Result<(), Box<dyn std::error::Error>>{
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if game.grid[y][x] {
                let block = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    ggez::graphics::Rect::new_i32(
                        (x as i32) * 25,
                        (y as i32) * 25,
                        25,
                        25,
                    ),
                    ggez::graphics::Color::from_rgb(100, 100, 255),
                )?;
                canvas.draw(&block, DrawParam::default());
            }
        }
        
    }
    Ok(())
}
