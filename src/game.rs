use ggez::event::EventHandler;
use ggez::graphics::{ DrawMode,  Mesh, DrawParam};
use crate::grid::{GRID_WIDTH, GRID_HEIGHT};
use ggez::input::keyboard::{KeyCode, KeyInput};
use crate::player;
use crate::grid;
use crate::utils;
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
    pub grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],
}

impl Game {
    pub fn new() -> Self {
        let enemies = enemy::create_enemies();
        let grid = grid::create_grid();

        Game {
            state: GameState::Menu,
            score: 0,
            player: player::Player::new(400.0, 240.0),
            enemies,
            grid,
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.player = player::Player::new(400.0, 240.0);
    }

}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.state {
            GameState::Menu => {
                // Menu logic
            }
            GameState::Play => {
                self.player.update_position();
                self.enemies.iter_mut().for_each(|enemy| enemy.update(&self.grid));

                if utils::check_collision(&self.grid,self.player.pos.0, self.player.pos.1 + 15.0) {
                    self.player.velocity.1 = 0.0;
                } else {
                    self.player.velocity.1 += 0.5; // gravity
                }

                if self.player.pos.1 > 460.0 {
                    self.player.pos.1 = 460.0;
                    self.player.velocity.1 = 0.0;
                }

                // Collision with enemies
                for enemy in &mut self.enemies {
                    if (self.player.pos.0 - enemy.pos.0).abs() < 20.0
                        && (self.player.pos.1 - enemy.pos.1).abs() < 20.0
                    {
                        self.state = GameState::GameOver;
                    }
                }
            }
            GameState::GameOver => {
                // Game Over logic
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK);

        match self.state {
            GameState::Menu => {
                let menu_text = ggez::graphics::Text::new("Press SPACE to Start");
                canvas.draw(
                    &menu_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 300.0, y: 200.0 }),
                );
            }
            GameState::Play => {
                let _=grid::draw(&mut canvas, self, ctx);
   
                let _=player::Player::draw(&mut canvas, self, ctx);

                let _=enemy::Enemy::draw(&mut canvas, self, ctx);
            

                let score_text = ggez::graphics::Text::new(format!("Score: {}", self.score));
                canvas.draw(
                    &score_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 10.0, y: 10.0 }),
                );
            }
            GameState::GameOver => {
                let over_text = ggez::graphics::Text::new("Game Over! Press SPACE to Restart");
                canvas.draw(
                    &over_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 250.0, y: 200.0 }),
                );
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _: &mut ggez::Context, input: KeyInput, _: bool) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Space => match self.state {
                    GameState::Menu => self.state = GameState::Play,
                    GameState::GameOver => {
                        self.state = GameState::Menu;
                        self.reset();
                    }
                    _=>{}
                },
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
                        self.player.velocity.1 = -15.0;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    fn key_up_event(&mut self, _: &mut ggez::Context, input: KeyInput)->ggez::GameResult {
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
