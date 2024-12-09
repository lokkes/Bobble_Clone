use ggez::event::EventHandler;
use ggez::graphics::{ DrawMode, Mesh, DrawParam };
use crate::grid::{ GridConfig, GRID_HEIGHT, GRID_WIDTH };
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
    pub player_images: Vec<graphics::Image>,
    pub player_state: player::PlayerState,
    pub current_frame: usize,
    pub frame_timer: f32,
    pub grid_image: graphics::Image,
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
                (0, 1, 8), // Top line
                (0, 12, 20),
                (0, 24, 32),
                (9, 5, 27), // Middle line
                (17, 1, 8), // Bottom lines
                (17, 12, 20),
                (17, 24, 32),
                (13, 1, 11), // Bottom middle blocks
                (13, 21, 32),
                (5, 5, 10), // Floating block above middle platform
                (5, 22, 27) // Another floating block
            ],
        };

        let enemies = enemy::create_enemies();
        let grid = grid::create_grid(&level1_config);
        let player_images = vec![
            graphics::Image::from_path(ctx, "/still.png").unwrap(),
            graphics::Image::from_path(ctx, "/run00.png").unwrap(),
            graphics::Image::from_path(ctx, "/run01.png").unwrap(),
            graphics::Image::from_path(ctx, "/run02.png").unwrap(),
            graphics::Image::from_path(ctx, "/run03.png").unwrap(),
            graphics::Image::from_path(ctx, "/run10.png").unwrap(),
            graphics::Image::from_path(ctx, "/run11.png").unwrap(),
            graphics::Image::from_path(ctx, "/run12.png").unwrap(),
            graphics::Image::from_path(ctx, "/run13.png").unwrap(),
            graphics::Image::from_path(ctx, "/jump0.png").unwrap()
        ];
        let grid_image = graphics::Image::from_path(ctx, "/block0.png").unwrap();

        Game {
            state: GameState::Menu,
            score: 0,
            player: player::Player::new(400.0, 240.0),
            enemies,
            bullets: vec![],
            grid,
            enemy_spawn_timer: 10.0,
            player_images,
            player_state: player::PlayerState::Idle,
            grid_image,
            current_frame: 0,
            frame_timer: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.player = player::Player::new(400.0, 240.0);
        self.enemies = enemy::create_enemies();
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
                    });

                    // Timer zurücksetzen
                    self.enemy_spawn_timer = 10.0;
                }

                // Aktualisiere bestehende Gegner
                for enemy in &mut self.enemies {
                    enemy.update(&self.grid);
                }

                player::Player::update_position(self, ctx);
                // self.player.update_position();
                self.enemies.iter_mut().for_each(|enemy| enemy.update(&self.grid));

                if self.player.pos.1 > 460.0 {
                    self.player.pos.1 = 460.0;
                    self.player.velocity.1 = 0.0;
                }

                // Collision with enemies
                for enemy in &mut self.enemies {
                    if
                        (self.player.pos.0 - enemy.pos.0).abs() < 20.0 &&
                        (self.player.pos.1 - enemy.pos.1).abs() < 20.0
                    {
                        self.state = GameState::GameOver;
                    }
                }

                for bullet in &mut self.bullets {
                    bullet.update();
                }

                // Entferne Kugeln, die aus dem Bildschirm verschwinden
                self.bullets.retain(|bullet| !bullet.is_off_screen());

                self.bullets.retain(|bullet| {
                    let mut hit_enemy = false;
                    self.enemies.retain(|enemy| {
                        let collision =
                            (bullet.pos.0 - enemy.pos.0).abs() < 15.0 &&
                            (bullet.pos.1 - enemy.pos.1).abs() < 15.0;
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
                let menu_text = ggez::graphics::Text::new("Press SPACE to Start");
                canvas.draw(
                    &menu_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 300.0, y: 200.0 })
                );
            }
            GameState::Play => {
                let _ = grid::draw(&mut canvas, self);

                let _ = player::Player::draw(&mut canvas, self);

                let _ = enemy::Enemy::draw(&mut canvas, self, ctx);

                let score_text = ggez::graphics::Text::new(format!("Score: {}", self.score));
                canvas.draw(
                    &score_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 10.0, y: 10.0 })
                );

                for bullet in &self.bullets {
                    let bullet_mesh = Mesh::new_circle(
                        ctx,
                        DrawMode::fill(),
                        ggez::mint::Point2 { x: 0.0, y: 0.0 },
                        5.0,
                        0.1,
                        ggez::graphics::Color::from_rgb(255, 255, 0) // Gelbe Kugeln
                    )?;
                    canvas.draw(
                        &bullet_mesh,
                        DrawParam::default().dest(ggez::mint::Point2 {
                            x: bullet.pos.0,
                            y: bullet.pos.1,
                        })
                    );
                }
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
        _: &mut ggez::Context,
        input: KeyInput,
        _: bool
    ) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Space =>
                    match self.state {
                        GameState::Menu => {
                            self.state = GameState::Play;
                        }
                        GameState::GameOver => {
                            self.state = GameState::Menu;
                            self.reset();
                        }
                        GameState::Play => {
                            // Kugel erstellen
                            let velocity = if self.player.view_right {
                                (1.0, 0.0)
                            } else {
                                (-1.0, 0.0)
                            };
                            self.bullets.push(bullet::Bullet {
                                pos: (self.player.pos.0, self.player.pos.1 - 10.0),
                                velocity,
                            });
                        }
                    }
                KeyCode::Left => {
                    if self.state == GameState::Play {
                        self.player.velocity.0 = -5.0;
                    }
                }
                KeyCode::Right => {
                    if self.state == GameState::Play {
                        self.player.velocity.0 = 5.0;
                    }
                }
                KeyCode::Up => {
                    if self.state == GameState::Play {
                        self.player.velocity.1 = -11.0;
                    }
                }
                _ => {}
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
