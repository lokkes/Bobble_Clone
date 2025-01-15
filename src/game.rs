use std::process::exit;

use ggez::audio::{SoundSource, Source};
use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use crate::grid::{ GridConfig, GRID_HEIGHT, GRID_WIDTH };
use crate::resources::Resources;
use ggez::input::keyboard::{ KeyCode, KeyInput };
use crate::{ player, utils };
use crate::grid;
use crate::enemy;
use crate::bullet;
use crate::bubble;
use crate::enemy_bullet;
use ggez::graphics;

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Play,
    GameOver,
}

pub struct Game {
    pub state: GameState,
    pub score: i32,
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub bullets: Vec<bullet::Bullet>,
    pub bubbles: Vec<bubble::Bubble>,
    pub enemy_bullets: Vec<enemy_bullet::EnemyBullet>,
    pub grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],
    pub enemy_spawn_timer: f32,
    pub resources: Resources,
    pub window_width: f32,
    pub window_height: f32,
    pub selected_menu_option: usize,
    pub selected_window_size: usize,
    pub window_sizes: Vec<(f32, f32)>,
    pub block_size: f32,
    pub music: Source,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let level1_config = GridConfig {
            vertical_lines: vec![
                (0, 0, GRID_HEIGHT),
                (1, 0, GRID_HEIGHT),
                (GRID_WIDTH - 2, 0, GRID_HEIGHT),
                (GRID_WIDTH - 1, 0, GRID_HEIGHT)
            ],
            horizontal_lines: vec![
                (0, 1, 8),
                (0, 12, 20),
                (0, 24, 32),
                (5, 5, 10),
                (5, 22, 27),
                (9, 5, 27),
                (13, 1, 11),
                (13, 21, 32),
                (17, 1, 8),
                (17, 12, 20),
                (17, 24, 32)
            ],
        };

        let grid = grid::create_grid(&level1_config);
        let (width, height) = ctx.gfx.drawable_size();
        let block_size = width / (GRID_WIDTH as f32);
        let enemies = enemy::create_enemies(width, height, block_size);
        let resources = Resources::load(ctx);
        let mut music = Source::new(ctx, "/sounds/theme.ogg").unwrap();
        music.set_repeat(true);

        Game {
            state: GameState::Menu,
            score: 0,
            player: player::Player::new(400.0, 240.0),
            enemies,
            bullets: vec![],
            bubbles: vec![],
            enemy_bullets: vec![],
            grid,
            enemy_spawn_timer: 10.0,
            resources,
            window_width: width,
            window_height: height,
            selected_menu_option: 0,
            selected_window_size: 0,
            window_sizes: vec![(800.0, 480.0), (1024.0, 768.0), (1280.0, 720.0), (1920.0, 1080.0)],
            block_size,
            music
        }
    }

    pub fn reset(&mut self, _ctx: &mut ggez::Context) {
        self.score = 0;
        self.player = player::Player::new(self.window_width / 2.0, self.window_width / 2.0);
        self.enemies = enemy::create_enemies(
            self.window_width,
            self.window_height,
            self.block_size
        );
        self.bullets = vec![];
        self.bubbles = vec![];
        self.enemy_bullets = vec![];
    }

    fn set_window_size(&mut self, ctx: &mut ggez::Context) {
        let (width, height) = self.window_sizes[self.selected_window_size];
        self.window_width = width;
        self.window_height = height;
        self.block_size = width / (GRID_WIDTH as f32);
        ctx.gfx.set_drawable_size(width, height).unwrap();
    }

    pub fn handle_collisions(&mut self) {
        // collision Player and Enemy
        for enemy in &mut self.enemies {
            if
                (self.player.pos.0 - enemy.pos.0).abs() < self.block_size &&
                (self.player.pos.1 - enemy.pos.1).abs() < self.block_size
            {
                self.state = GameState::GameOver;
            }
        }

        // collision Bullets and Enemy
        self.bullets.retain(|bullet| {
            let mut hit_enemy = false;
            self.enemies.retain(|enemy| {
                let collision =
                    (bullet.pos.0 - enemy.pos.0).abs() < self.block_size &&
                    (bullet.pos.1 - enemy.pos.1).abs() < self.block_size * 2.0;
                if collision {
                    self.score += 10;
                    hit_enemy = true;
                    self.bubbles.push(
                        bubble::Bubble::new(enemy.pos, (0.0, -self.block_size / 25.0))
                    );
                }
                !collision
            });
            !hit_enemy
        });

        //collision enemy_bullets and player
        for bullet in &self.enemy_bullets {
            if
                (bullet.pos.0 - self.player.pos.0).abs() < self.block_size &&
                (bullet.pos.1 - self.player.pos.1).abs() < self.block_size
            {
                self.state = GameState::GameOver;
            }
        }

        //collision enemy_bullet and player_bullet
        self.bullets.retain(|bullet| {
            let mut hit_bullet = false;
            self.enemy_bullets.retain(|enemy_bullets| {
                let collision =
                    (bullet.pos.0 - enemy_bullets.pos.0).abs() < self.block_size &&
                    (bullet.pos.1 - enemy_bullets.pos.1).abs() < self.block_size * 2.0;
                if collision {
                    hit_bullet = true;
                }
                !collision
            });
            !hit_bullet
        });
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.state {
            GameState::Menu => {
                // Menülogik
            }
            GameState::Play => {
                let delta_time = ctx.time.delta().as_secs_f32();
                utils::update_objects(self, ctx, delta_time);
                self.handle_collisions();
            }
            GameState::GameOver => {}
        }

        // Framerate limitieren
        while ctx.time.check_update_time(60) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK);

        match self.state {
            GameState::Menu => {
                for (i, image) in self.resources.menu_images.clone().into_iter().enumerate() {
                    let x = self.window_width / 2.0 - (image.width() as f32) / 2.0;
                    let y = 150.0 + (i as f32) * 100.0;
                    let color = if i == self.selected_menu_option {
                        graphics::Color::WHITE // Highlighted option
                    } else {
                        graphics::Color::new(0.5, 0.5, 0.5, 1.0) // Gray for non-highlighted option
                    };

                    canvas.draw(
                        &image,
                        DrawParam::default().dest(ggez::mint::Point2 { x, y }).color(color)
                    );
                }

                if self.selected_menu_option == 1 {
                    let (width, height) = self.window_sizes[self.selected_window_size];
                    let text = graphics::Text::new(
                        format!(
                            "Window Size: {}x{} (Use Left/Right to change)",
                            width as u32,
                            height as u32
                        )
                    );
                    canvas.draw(
                        &text,
                        DrawParam::default().dest(ggez::mint::Point2 { x: 100.0, y: 300.0 })
                    );
                }
            }
            GameState::Play => {
                let _ = grid::draw(&mut canvas, self);
                let _ = player::Player::draw(&mut canvas, self);
                let _ = enemy::Enemy::draw(&mut canvas, self, ctx);
                let _ = bullet::Bullet::draw(&mut canvas, self);
                let _ = bubble::Bubble::draw(&mut canvas, self);
                let _ = enemy_bullet::EnemyBullet::draw(&mut canvas, self);

                let score_text = ggez::graphics::Text::new(format!("Score: {}", self.score));
                canvas.draw(
                    &score_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 10.0, y: 10.0 })
                );
            }
            GameState::GameOver => {
                canvas.draw(
                    &self.resources.game_over_image,
                    DrawParam::default()
                        .dest(ggez::mint::Point2 {
                            x: -self.block_size * 0.8,
                            y: -self.block_size * 0.8,
                        })
                        .scale(ggez::mint::Vector2 {
                            x: self.block_size / (GRID_WIDTH as f32) + self.block_size / 114.285,
                            y: self.block_size / (GRID_WIDTH as f32) + self.block_size / 114.285,
                        })
                );
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        input: KeyInput,
        _: bool
    ) -> ggez::GameResult {
        match self.state {
            GameState::Menu => {
                if let Some(keycode) = input.keycode {
                    match keycode {
                        KeyCode::Space => {
                            self.reset(ctx);
                            let _ = self.music.play(ctx);
                            self.state = GameState::Play;
                        }
                        KeyCode::Up => {
                            if self.selected_menu_option > 0 {
                                self.selected_menu_option -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_menu_option < self.resources.menu_images.len() - 1 {
                                self.selected_menu_option += 1;
                            }
                        }
                        KeyCode::Left => {
                            if self.selected_menu_option == 1 && self.selected_window_size > 0 {
                                self.selected_window_size -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if
                                self.selected_menu_option == 1 &&
                                self.selected_window_size < self.window_sizes.len() - 1
                            {
                                self.selected_window_size += 1;
                            }
                        }
                        KeyCode::Return => {
                            match self.selected_menu_option {
                                0 => {
                                    self.state = GameState::Play;
                                }
                                1 => {
                                    self.set_window_size(ctx);
                                }
                                2 => exit(0),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            GameState::GameOver => {
                if let Some(keycode) = input.keycode {
                    match keycode {
                        KeyCode::Space => {
                            self.state = GameState::Menu;
                            self.reset(ctx);
                        }
                        _ => {}
                    }
                }
            }
            GameState::Play => {
                if let Some(keycode) = input.keycode {
                    match keycode {
                        KeyCode::Space => {
                            // Bullet velocity based on player facing direction
                            let velocity = if self.player.view_right {
                                (self.block_size / 3.0, 0.0) // Bullet moves right
                            } else {
                                (-self.block_size / 3.0, 0.0) // Bullet moves left
                            };

                            self.bullets.push(bullet::Bullet {
                                pos: (self.player.pos.0, self.player.pos.1 - self.block_size * 1.1),
                                velocity,
                                image: self.resources.bullet_image.clone(),
                            });
                        }
                        KeyCode::Left => {
                            self.player.velocity.0 = -self.block_size / 5.0;
                            self.player.view_right = false;
                        }
                        KeyCode::Right => {
                            self.player.velocity.0 = self.block_size / 5.0;
                            self.player.view_right = true;
                        }
                        KeyCode::Up => {
                            if self.player.velocity.1 == 0.0 {
                                self.player.velocity.1 = -self.block_size / 2.4;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _: &mut ggez::Context, input: KeyInput) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            if self.state == GameState::Play {
                if
                    (keycode == KeyCode::Left && self.player.velocity.0 < 0.0) ||
                    (keycode == KeyCode::Right && self.player.velocity.0 > 0.0)
                {
                    self.player.velocity.0 = 0.0;
                }
            }
        }
        Ok(())
    }
}
