use crate::grid::{ GRID_WIDTH, GRID_HEIGHT, BLOCK_SIZE };
use ggez::graphics::DrawParam;
use crate::utils::check_collision;
use crate::game::Game;

#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    WalkingRight,
    WalkingLeft,
    Jumping,
}

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

    pub fn update_position(game: &mut Game, ctx: &mut ggez::Context) {
        game.player.velocity.1 += 0.5; // Gravitation

        if game.player.velocity.0 > 0.0 {
            game.player.view_right = true;
        }
        if game.player.velocity.0 < 0.0 {
            game.player.view_right = false;
        }
        // Horizontale Bewegung prüfen
        let next_x = game.player.pos.0 + game.player.velocity.0;
        if
            next_x <= 0.0 ||
            next_x >= (GRID_WIDTH as f32) * BLOCK_SIZE - BLOCK_SIZE ||
            check_collision(&game.grid, next_x, game.player.pos.1 + 15.0) ||
            check_collision(&game.grid, next_x + BLOCK_SIZE, game.player.pos.1 + 15.0) ||
            check_collision(&game.grid, next_x - BLOCK_SIZE, game.player.pos.1 + 15.0)
        {
            game.player.velocity.0 = 0.0;
        } else {
            game.player.pos.0 = next_x;
        }

        // Vertikale Bewegung prüfen
        let next_y = game.player.pos.1 + game.player.velocity.1;
        if
            next_y > (GRID_HEIGHT as f32) * BLOCK_SIZE ||
            (check_collision(&game.grid, game.player.pos.0, next_y + 15.0) &&
                game.player.velocity.1 >= 0.0)
        {
            game.player.velocity.1 = 0.0; // Gravitation stoppen
        } else {
            game.player.pos.1 = next_y;
        }

        game.player_state = match (game.player.velocity.1 < 0.0, game.player.velocity.0) {
            (true, _) => PlayerState::Jumping,
            (false, v) if v > 0.0 => PlayerState::WalkingRight,
            (false, v) if v < 0.0 => PlayerState::WalkingLeft,
            _ => PlayerState::Idle,
        };

        let delta_time = ctx.time.delta().as_secs_f32();
        game.frame_timer += delta_time;
        if
            game.frame_timer >= 0.1 &&
            (game.player_state == PlayerState::WalkingRight ||
                game.player_state == PlayerState::WalkingLeft)
        {
            game.current_frame = (game.current_frame + 1) % 4;
            game.frame_timer = 0.0;
        }
    }

    pub fn draw(canvas: &mut ggez::graphics::Canvas, game: &mut Game) {
        let player_image = match game.player_state {
            PlayerState::Idle => &game.player_images[0],
            PlayerState::WalkingLeft => &game.player_images[1 + game.current_frame],
            PlayerState::WalkingRight => &game.player_images[5 + game.current_frame],
            PlayerState::Jumping => &game.player_images[9],
        };

        canvas.draw(
            player_image,
            DrawParam::default().dest(ggez::mint::Point2 {
                x: game.player.pos.0 - 30.0,
                y: game.player.pos.1 - 50.0,
            })
        );
    }
}
