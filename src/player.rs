use crate::grid::GRID_WIDTH;
use ggez::graphics::DrawParam;
use crate::utils::{ check_collision_player, get_y_pos_correction };
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
    current_frame: usize,
    frame_timer: f32,
    player_state: PlayerState,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: (x, y),
            velocity: (0.0, 0.0),
            view_right: true,
            current_frame: 0,
            frame_timer: 0.0,
            player_state: PlayerState::Idle,
        }
    }

    pub fn update(game: &mut Game, ctx: &mut ggez::Context) {
        game.player.velocity.1 += game.block_size / 50.0; // Gravitation

        // Horizontale Bewegung prüfen
        let next_x = game.player.pos.0 + game.player.velocity.0;
        let next_y = game.player.pos.1 + game.player.velocity.1;
        if
            next_x <= 0.0 ||
            next_x >= game.window_width ||
            check_collision_player(
                &game.grid,
                next_x,
                game.player.pos.1 + game.block_size * (game.block_size / 114.285),
                game.block_size
            )
        {
            // game.player.velocity.0 = 0.0;
        } else {
            if next_y > game.window_height - game.block_size {
                game.player.pos.0 = game.player.pos.0 - game.player.velocity.0 * 0.2;
            } else {
                game.player.pos.0 = next_x;
            }
        }

        // Vertikale Bewegung prüfen
        if
            (check_collision_player(
                &game.grid,
                game.player.pos.0,
                next_y + game.block_size * (game.block_size / 114.285),
                game.block_size
            ) && game.player.velocity.1 >= 0.0) ||
            (next_y < game.block_size && game.player.velocity.1 < 0.0)
        {
            game.player.velocity.1 = 0.0; // Gravitation stoppen
        } else {
            game.player.pos.1 = next_y;
        }
        if next_y > game.window_height {
            game.player.pos.1 = 0.0;
        }

        game.player.player_state = match (game.player.velocity.1 < 0.0, game.player.velocity.0) {
            (true, _) => PlayerState::Jumping,
            (false, v) if v > 0.0 => PlayerState::WalkingRight,
            (false, v) if v < 0.0 => PlayerState::WalkingLeft,
            _ => PlayerState::Idle,
        };

        let delta_time = ctx.time.delta().as_secs_f32();
        game.player.frame_timer += delta_time;
        if
            game.player.frame_timer >= 0.1 &&
            (game.player.player_state == PlayerState::WalkingRight ||
                game.player.player_state == PlayerState::WalkingLeft)
        {
            game.player.current_frame = (game.player.current_frame + 1) % 4;
            game.player.frame_timer = 0.0;
        }
    }

    pub fn draw(canvas: &mut ggez::graphics::Canvas, game: &mut Game) {
        let player_image = match game.player.player_state {
            PlayerState::Idle => game.resources.player_images[0].clone(),
            PlayerState::WalkingLeft =>
                game.resources.player_images[1 + game.player.current_frame].clone(),
            PlayerState::WalkingRight =>
                game.resources.player_images[5 + game.player.current_frame].clone(),
            PlayerState::Jumping => game.resources.player_images[9].clone(),
        };

        canvas.draw(
            &player_image,
            DrawParam::default()
                .dest(ggez::mint::Point2 {
                    x: game.player.pos.0 - game.block_size * 1.4,
                    y: game.player.pos.1 -
                    get_y_pos_correction(game.window_width, game.block_size, &player_image),
                })
                .scale(ggez::mint::Vector2 {
                    x: game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285,
                    y: game.block_size / (GRID_WIDTH as f32) + game.block_size / 114.285,
                })
        );
    }
}
