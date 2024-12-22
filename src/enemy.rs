use crate::grid::{ GRID_WIDTH, GRID_HEIGHT };
use crate::utils::{ check_collision, get_y_pos_correction };
use ggez::graphics::DrawParam;
use crate::game::Game;

pub struct Enemy {
    pub pos: (f32, f32), //(pos_x,pos_y)
    pub velocity: (f32, f32),
}

pub fn create_enemies(window_width: f32, window_height: f32, block_size: f32) -> Vec<Enemy> {
    vec![
        Enemy {
            pos: (window_width / 8.0, window_height / 2.0),
            velocity: (block_size / 15.0, block_size / 7.0),
        },
        Enemy {
            pos: (window_width / 4.0, 00.0),
            velocity: (block_size / 8.0, 0.0),
        },
        Enemy {
            pos: (window_width / 4.0, 00.0),
            velocity: (-block_size / 15.0, 0.0),
        }
    ]
}

impl Enemy {
    pub fn new(pos: (f32, f32), velocity: (f32, f32)) -> Self {
        Enemy {
            pos,
            velocity,
        }
    }

    pub fn update(&mut self, grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT], block_size: f32) {
        self.velocity.1 += block_size / 50.0; // Gravitation

        // Horizontale Bewegung prüfen
        let next_x = self.pos.0 + self.velocity.0;
        if
            next_x <= 0.0 ||
            next_x >= (GRID_WIDTH as f32) * block_size ||
            check_collision(
                grid,
                next_x,
                self.pos.1 + block_size * (block_size / 114.285),
                block_size
            )
        {
            self.velocity.0 = -self.velocity.0; // Richtung umkehren
        } else {
            self.pos.0 = next_x;
        }

        // Vertikale Bewegung prüfen
        let next_y = self.pos.1 + self.velocity.1;
        if
            check_collision(
                grid,
                self.pos.0,
                next_y + block_size * (block_size / 114.285),
                block_size
            )
        {
            self.velocity.1 = 0.0; // Gravitation stoppen
        } else {
            self.pos.1 = next_y;
        }
    }

    pub fn draw(
        canvas: &mut ggez::graphics::Canvas,
        game: &mut Game,
        _ctx: &mut ggez::Context
    ) -> Result<(), Box<dyn std::error::Error>> {
        for enemy in &game.enemies {
            let image = if enemy.velocity.0 < 0.0 {
                &game.resources.enemy_images[0]
            } else {
                &game.resources.enemy_images[1]
            };
            canvas.draw(
                image,
                DrawParam::default()
                    .dest(ggez::mint::Point2 {
                        x: enemy.pos.0 - game.block_size,
                        y: enemy.pos.1 -
                        get_y_pos_correction(game.window_width, game.block_size, image),
                    })
                    .scale(ggez::mint::Vector2 {
                        x: game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285,
                        y: game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285,
                    })
            );
        }
        Ok(())
    }

    pub fn is_off_screen(&self, block_size: f32) -> bool {
        self.pos.0 < 0.0 ||
            self.pos.0 > (GRID_WIDTH as f32) * block_size + 2.0 * block_size ||
            self.pos.1 < 0.0 ||
            self.pos.1 > (GRID_HEIGHT as f32) * block_size + 2.0 * block_size
    }
}
