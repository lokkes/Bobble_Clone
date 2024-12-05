use crate::grid::{ GRID_WIDTH, GRID_HEIGHT, BLOCK_SIZE };
use ggez::graphics::DrawParam;
use crate::utils::check_collision;
use crate::game::Game;

pub struct Player {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
    pub view_right: bool,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: (x, y),
            velocity: (0.0, 0.0),
            view_right: true,
        }
    }

    pub fn update_position(&mut self, grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {
        self.velocity.1 += 0.5; // Gravitation

        if self.velocity.0 > 0.0 {
            self.view_right = true;
        }
        if self.velocity.0 < 0.0 {
            self.view_right = false;
        }
        // Horizontale Bewegung prüfen
        let next_x = self.pos.0 + self.velocity.0;
        if
            next_x <= 0.0 ||
            next_x >= (GRID_WIDTH as f32) * BLOCK_SIZE - BLOCK_SIZE ||
            check_collision(grid, next_x, self.pos.1 + 15.0)
        {
            self.velocity.0 = 0.0;
        } else {
            self.pos.0 = next_x;
        }

        // Vertikale Bewegung prüfen
        let next_y = self.pos.1 + self.velocity.1;
        if
            next_y > (GRID_HEIGHT as f32) * BLOCK_SIZE ||
            (check_collision(grid, self.pos.0, next_y + 15.0) && self.velocity.1 >= 0.0)
        {
            self.velocity.1 = 0.0; // Gravitation stoppen
        } else {
            self.pos.1 = next_y;
        }
    }

    pub fn draw(canvas: &mut ggez::graphics::Canvas, game: &mut Game) {
        canvas.draw(
            &game.player_image,
            DrawParam::default().dest(ggez::mint::Point2 {
                x: game.player.pos.0 - 30.0,
                y: game.player.pos.1 - 50.0,
            })
        );
    }
}
