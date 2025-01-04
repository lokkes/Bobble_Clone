use ggez::graphics::Image;
use rand::Rng;

use crate::{ enemy, enemy_bullet, game::Game, grid::{ GRID_HEIGHT, GRID_WIDTH }, player, utils };

pub fn check_collision(
    grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT],
    x: f32,
    y: f32,
    block_size: f32
) -> bool {
    let grid_x = (x / block_size) as usize;
    let grid_y = (y / block_size) as usize;

    if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
        grid[grid_y][grid_x]
    } else {
        false
    }
}

pub fn check_collision_player(
    grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT],
    x: f32,
    y: f32,
    block_size: f32
) -> bool {
    check_collision(grid, x, y, block_size) ||
        check_collision(grid, x + block_size, y, block_size) ||
        check_collision(grid, x - block_size, y, block_size)
}

pub fn get_y_pos_correction(window_width: f32, block_size: f32, image: &Image) -> f32 {
    let y_pos_correction;
    if window_width >= 1920.0 {
        y_pos_correction =
            ((image.height() as f32) * block_size) / (GRID_WIDTH as f32) + block_size / 114.285;
    } else {
        y_pos_correction = block_size * 2.5;
    }
    y_pos_correction
}

pub fn update_objects(game: &mut Game, ctx: &mut ggez::Context, delta_time: f32) {
    //Player
    player::Player::update(game, ctx);

    //Enemy
    game.enemy_spawn_timer -= delta_time;
    if game.enemy_spawn_timer <= 0.0 {
        game.enemies.push(
            enemy::Enemy::new((100.0 + (game.enemies.len() as f32) * 50.0, 100.0), (1.0, 0.0))
        );
        game.enemy_spawn_timer = 10.0; // Timer zurücksetzen
    }
    game.enemies.iter_mut().for_each(|enemy| enemy.update(&game.grid, game.block_size));
    game.enemies.retain(|enemy| !enemy.is_off_screen(game.block_size));

    //Bullets
    game.bullets.iter_mut().for_each(|bullet| bullet.update());
    game.bullets.retain(|bullet| !bullet.is_off_screen(game.block_size));

    //Bubbles
    game.bubbles.iter_mut().for_each(|bubble| bubble.update(ctx));

    // //enemy_bubbles
    for enemy in &game.enemies {
        if utils::random_f32() < 0.005 {
            game.enemy_bullets.push(enemy_bullet::EnemyBullet {
                pos: enemy.pos,
                velocity: (
                    if enemy.velocity.0 > 0.0 {
                        game.block_size / 3.0
                    } else {
                        -(game.block_size / 3.0)
                    },
                    0.0,
                ),
            });
        }
    }
    game.enemy_bullets.iter_mut().for_each(|bullet| bullet.update());
    game.enemy_bullets.retain(|bullet| !bullet.is_off_screen());
}

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}
