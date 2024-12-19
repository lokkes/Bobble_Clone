use std::process::exit;

use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use crate::grid::{ GridConfig, GRID_HEIGHT, GRID_WIDTH };
use crate::resources::Resources;
use ggez::input::keyboard::{ KeyCode, KeyInput };
use crate::player;
use crate::grid;
use crate::enemy;
use crate::bullet;
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
    pub grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],
    pub enemy_spawn_timer: f32,
    pub resources: Resources, // Neue Ressourcensammlung
    pub player_state: player::PlayerState,
    pub current_frame: usize,
    pub frame_timer: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub selected_option: usize, // Aktuelle Menüoption (Index)
    pub menu_options: Vec<&'static str>, // Menüoptionen
    pub selected_size: usize, // Ausgewählte Fenstergröße
    pub window_sizes: Vec<(f32, f32)>,
    pub block_size: f32,
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
        let enemies = enemy::create_enemies(ctx, width, height, block_size);
        let resources = Resources::load(ctx);

        Game {
            state: GameState::Menu,
            score: 0,
            player: player::Player::new(400.0, 240.0),
            enemies,
            bullets: vec![],
            grid,
            enemy_spawn_timer: 10.0,
            resources, // Ressourcen verwenden
            player_state: player::PlayerState::Idle,
            current_frame: 0,
            frame_timer: 0.0,
            window_width: width,
            window_height: height,
            selected_option: 0,
            menu_options: vec!["Start Game", "Set Window Size", "Exit"],
            selected_size: 0,
            window_sizes: vec![(800.0, 480.0), (1024.0, 768.0), (1280.0, 720.0), (1920.0, 1080.0)],
            block_size,
        }
    }

    pub fn reset(&mut self, ctx: &mut ggez::Context) {
        self.score = 0;
        self.player = player::Player::new(self.window_width / 2.0, self.window_width / 2.0);
        self.enemies = enemy::create_enemies(
            ctx,
            self.window_width,
            self.window_height,
            self.block_size
        );
        self.bullets = vec![];
    }

    fn set_window_size(&mut self, ctx: &mut ggez::Context) {
        let (width, height) = self.window_sizes[self.selected_size];
        self.window_width = width;
        self.window_height = height;
        self.block_size = width / (GRID_WIDTH as f32);
        ctx.gfx.set_drawable_size(width, height).unwrap();
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.state {
            GameState::Menu => {
                // Menu logic
            }
            GameState::Play => {
                let delta_time = ctx.time.delta().as_secs_f32();
                // Timer aktualisieren
                self.enemy_spawn_timer -= delta_time;

                if self.enemy_spawn_timer <= 0.0 {
                    // Füge einen neuen Gegner hinzu
                    self.enemies.push(enemy::Enemy {
                        pos: (100.0 + (self.enemies.len() as f32) * 50.0, 100.0),
                        velocity: (1.0, 0.0),
                        left_image: ggez::graphics::Image::from_path(ctx, "/robot000.png").unwrap(),
                        right_image: ggez::graphics::Image
                            ::from_path(ctx, "/robot010.png")
                            .unwrap(),
                    });

                    // Timer zurücksetzen
                    self.enemy_spawn_timer = 10.0;
                }

                // Aktualisiere bestehende Gegner
                for enemy in &mut self.enemies {
                    enemy.update(&self.grid, self.block_size);
                }

                self.enemies.retain(|enemy| !enemy.is_off_screen(self.block_size));
                player::Player::update_position(self, ctx);

                // Collision with enemies
                for enemy in &mut self.enemies {
                    if
                        (self.player.pos.0 - enemy.pos.0).abs() < self.block_size &&
                        (self.player.pos.1 - enemy.pos.1).abs() < self.block_size
                    {
                        self.state = GameState::GameOver;
                    }
                }

                for bullet in &mut self.bullets.iter_mut() {
                    bullet.update();
                }

                // Entferne Kugeln, die aus dem Bildschirm verschwinden
                self.bullets.retain(|bullet| !bullet.is_off_screen(self.block_size));

                self.bullets.retain(|bullet| {
                    let mut hit_enemy = false;
                    self.enemies.retain(|enemy| {
                        let collision =
                            (bullet.pos.0 - enemy.pos.0).abs() < self.block_size &&
                            (bullet.pos.1 - enemy.pos.1).abs() < self.block_size * 2.0;
                        if collision {
                            self.score += 10;
                            hit_enemy = true;
                        }
                        !collision
                    });
                    !hit_enemy
                });
            }
            GameState::GameOver => {
                // Game Over logic
            }
        }
        // Framerate limitieren
        while ctx.time.check_update_time(60) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK);

        match self.state {
            GameState::Menu => {
                canvas.draw(
                    &self.resources.press_space_image,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 150.0, y: 150.0 })
                );
                for (i, option) in self.menu_options.iter().enumerate() {
                    let color = if i == self.selected_option {
                        graphics::Color::WHITE
                    } else {
                        graphics::Color::BLUE
                    };
                    let text = graphics::Text::new((*option).to_string());
                    canvas.draw(
                        &text,
                        DrawParam::default()
                            .dest(ggez::mint::Point2 { x: 100.0, y: 100.0 + (i as f32) * 50.0 })
                            .color(color)
                    );
                }

                // Wenn Fenstergröße ändern ausgewählt ist, zeige die aktuelle Größe an
                if self.selected_option == 1 {
                    let (width, height) = self.window_sizes[self.selected_size];
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

                let score_text = ggez::graphics::Text::new(format!("Score: {}", self.score));
                canvas.draw(
                    &score_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 10.0, y: 10.0 })
                );
            }
            GameState::GameOver => {
                let over_text = ggez::graphics::Text::new("Game Over! Press SPACE to Restart");
                canvas.draw(
                    &over_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 250.0, y: 200.0 })
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
                            self.state = GameState::Play;
                        }
                        KeyCode::Up => {
                            if self.selected_option > 0 {
                                self.selected_option -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_option < self.menu_options.len() - 1 {
                                self.selected_option += 1;
                            }
                        }
                        KeyCode::Left => {
                            if self.selected_option == 1 && self.selected_size > 0 {
                                self.selected_size -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if
                                self.selected_option == 1 &&
                                self.selected_size < self.window_sizes.len() - 1
                            {
                                self.selected_size += 1;
                            }
                        }
                        KeyCode::Return => {
                            match self.selected_option {
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

                            // Bullet image based on player facing direction
                            let bullet_image = if self.player.view_right {
                                self.resources.bullet_right_image.clone()
                            } else {
                                self.resources.bullet_left_image.clone()
                            };

                            self.bullets.push(bullet::Bullet {
                                pos: (self.player.pos.0, self.player.pos.1 - self.block_size * 1.1),
                                velocity,
                                image: bullet_image,
                            });
                        }
                        KeyCode::Left => {
                            if self.state == GameState::Play {
                                self.player.velocity.0 = -self.block_size / 5.0;
                                self.player.view_right = false; // Player faces left
                            }
                        }
                        KeyCode::Right => {
                            if self.state == GameState::Play {
                                self.player.velocity.0 = self.block_size / 5.0;
                                self.player.view_right = true; // Player faces right
                            }
                        }
                        KeyCode::Up => {
                            if self.state == GameState::Play {
                                if self.player.velocity.1 == 0.0 {
                                    self.player.velocity.1 = -self.block_size / 2.4;
                                }
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
                if keycode == KeyCode::Left || keycode == KeyCode::Right {
                    self.player.velocity.0 = 0.0;
                }
            }
        }
        Ok(())
    }
}
