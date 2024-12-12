use crate::grid::{ GRID_WIDTH, GRID_HEIGHT, BLOCK_SIZE };
use crate::utils::check_collision;
use ggez::graphics::DrawParam;
use crate::game::Game;

pub struct Enemy {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
    pub left_image: ggez::graphics::Image,
    pub right_image: ggez::graphics::Image,
}

pub fn create_enemies(ctx: &mut ggez::Context) -> Vec<Enemy> {
    vec![
        Enemy {
            pos: (100.0, 300.0),
            velocity: (0.5, 3.5),
            left_image: ggez::graphics::Image::from_path(ctx, "/robot000.png").unwrap(),
            right_image: ggez::graphics::Image::from_path(ctx, "/robot010.png").unwrap(),
        },
        Enemy {
            pos: (300.0, 100.0),
            velocity: (1.5, 0.0),
            left_image: ggez::graphics::Image::from_path(ctx, "/robot100.png").unwrap(),
            right_image: ggez::graphics::Image::from_path(ctx, "/robot110.png").unwrap(),
        },
        Enemy {
            pos: (500.0, 100.0),
            velocity: (0.5, 0.0),
            left_image: ggez::graphics::Image::from_path(ctx, "/robot000.png").unwrap(),
            right_image: ggez::graphics::Image::from_path(ctx, "/robot010.png").unwrap(),
        }
    ]
}

impl Enemy {
    pub fn update(&mut self, grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {
        self.velocity.1 += 0.5; // Gravitation

        // Horizontale Bewegung prüfen
        let next_x = self.pos.0 + self.velocity.0;
        if
            next_x <= 0.0 ||
            next_x >= (GRID_WIDTH as f32) * BLOCK_SIZE ||
            check_collision(grid, next_x, self.pos.1 + 15.0)
        {
            self.velocity.0 = -self.velocity.0; // Richtung umkehren
        } else {
            self.pos.0 = next_x;
        }

        // Vertikale Bewegung prüfen
        let next_y = self.pos.1 + self.velocity.1;
        if
            next_y > (GRID_HEIGHT as f32) * BLOCK_SIZE ||
            check_collision(grid, self.pos.0, next_y + 15.0)
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
            let image = if enemy.velocity.0 < 0.0 { &enemy.left_image } else { &enemy.right_image };
            canvas.draw(
                image,
                DrawParam::default().dest(ggez::mint::Point2 {
                    x: enemy.pos.0 - 30.0,
                    y: enemy.pos.1 - 50.0,
                })
            );
        }
        Ok(())
    }
}
