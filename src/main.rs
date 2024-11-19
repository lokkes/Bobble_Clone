use ggez::{Context, ContextBuilder,GameResult};
use ggez::event::{self, EventHandler};
use ggez::conf::WindowMode;// for changing windowsize 
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Text, DrawParam};


#[derive(PartialEq)]
enum GameState {
    Menu,
    Play,
    GameOver,
}

const GRID_WIDTH: usize = 28;  // Anzahl der Spalten
const GRID_HEIGHT: usize = 18; // Anzahl der Zeilen
const BLOCK_SIZE: f32 = 25.0;  // Größe jedes Blocks in Pixeln

struct Game {
    state: GameState,
    score: i32,
    player_pos: (f32, f32),
    player_velocity: (f32, f32),
    enemies: Vec<Enemy>,
    grid: [[bool; GRID_WIDTH]; GRID_HEIGHT],
}

impl Game {
    fn new() -> Self {
        let mut enemies = Vec::new();
        let predefined_positions = [(100.0, 100.0), (300.0, 200.0), (500.0, 150.0)];
        let predefined_velocities = [(1.0, -0.5), (-1.0, 0.0), (0.5, 0.0)];
        for i in 0..3 {
            enemies.push(Enemy {
                pos: predefined_positions[i],
                velocity: predefined_velocities[i],
            });
        }

        let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
        for x in 5..10 {
            grid[16][x] = true; // horizontale Linie bei Zeile 10
        }
        for y in 5..8 {
            grid[y][15] = true; // vertikale Linie bei Spalte 15
        }

        Game {
            state: GameState::Menu,
            score: 0,
            player_pos: (400.0, 240.0),
            player_velocity: (0.0, 0.0),
            enemies,
            grid,
        }
    }

    fn reset(&mut self) {
        self.score = 0;
        self.player_pos = (400.0, 240.0);
        self.player_velocity = (0.0, 0.0);
    }

    fn check_collision(&self, x: f32, y: f32) -> bool {
        let grid_x = (x / BLOCK_SIZE) as usize;
        let grid_y = (y / BLOCK_SIZE) as usize;

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            self.grid[grid_y][grid_x]
        } else {
            false
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            GameState::Menu => {
                // Menü-Logik
            }
            GameState::Play => {
                // Spielerbewegung
                let (vx, vy) = self.player_velocity;
                self.player_pos.0 += vx;
                self.player_pos.1 += vy;

                if self.check_collision(self.player_pos.0, self.player_pos.1 + 15.0) {
                    self.player_velocity.1 = 0.0; 
                    // self.player_pos.1 = (self.player_pos.1 / BLOCK_SIZE).floor() * BLOCK_SIZE;
                }
                else{
                    self.player_velocity.1 += 0.5; // Schwerkraft
                }

                // Bodenbegrenzung
                if self.player_pos.1 > 460.0 {
                    self.player_pos.1 = 460.0;
                    self.player_velocity.1 = 0.0; 
                }

                //Kollisionserkennung
                for enemy in &mut self.enemies {
                    enemy.update(&self.grid);

                    if (self.player_pos.0 - enemy.pos.0).abs() < 20.0
                        && (self.player_pos.1 - enemy.pos.1).abs() < 20.0
                    {
                        self.state = GameState::GameOver;
                    }
                }
                // Spielmechanik hier einfügen...
            }
            GameState::GameOver => {
                // Logik für "Game Over"
            }
        }
        // Framerate limitieren
        while ctx.time.check_update_time(60) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        match self.state {
            GameState::Menu => {
                let menu_text = Text::new("Press SPACE to Start");
                canvas.draw(
                    &menu_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 300.0, y: 200.0 }),
                );
            }
            GameState::Play => {
                // Hindernisse zeichnen
                for y in 0..GRID_HEIGHT {
                    for x in 0..GRID_WIDTH {
                        if self.grid[y][x] {
                            let block = Mesh::new_rectangle(
                                ctx,
                                DrawMode::fill(),
                                graphics::Rect::new_i32(
                                    (x as i32) * (BLOCK_SIZE as i32),
                                    (y as i32) * (BLOCK_SIZE as i32),
                                    BLOCK_SIZE as i32,
                                    BLOCK_SIZE as i32,
                                ),
                                Color::from_rgb(100, 100, 255),
                            )?;
                            canvas.draw(&block, DrawParam::default());
                        }
                    }
                }

                // Spieler zeichnen
                let player = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    ggez::mint::Point2 { x: 0.0, y: 0.0 },
                    15.0,
                    0.1,
                    Color::from_rgb(255, 255, 255),
                )?;
                canvas.draw(
                    &player,
                    DrawParam::default().dest(ggez::mint::Point2 {
                        x: self.player_pos.0,
                        y: self.player_pos.1,
                    }),
                );

                // Gegner zeichnen
                for enemy in &self.enemies {
                    let enemy_mesh = Mesh::new_circle(
                        ctx,
                        DrawMode::fill(),
                        ggez::mint::Point2 { x: 0.0, y: 0.0 },
                        15.0,
                        0.1,
                        Color::from_rgb(255, 0, 0), 
                    )?;
                    canvas.draw(
                        &enemy_mesh,
                        DrawParam::default().dest(ggez::mint::Point2 {
                            x: enemy.pos.0,
                            y: enemy.pos.1,
                        }),
                    );
                }

                // Punkte anzeigen
                let score_text = Text::new(format!("Score: {}", self.score));
                canvas.draw(
                    &score_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 10.0, y: 10.0 }),
                );
            }
            GameState::GameOver => {
                let over_text = Text::new("Game Over! Press SPACE to Restart");
                canvas.draw(
                    &over_text,
                    DrawParam::default().dest(ggez::mint::Point2 { x: 250.0, y: 200.0 }),
                );
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, input: KeyInput, _: bool) ->GameResult{
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Space => match self.state {
                    GameState::Menu => self.state = GameState::Play,
                    GameState::GameOver => {
                        self.state = GameState::Menu;
                        self.reset();
                    }
                    _ => {}
                },
                KeyCode::Left => {
                    if self.state == GameState::Play {
                        self.player_velocity.0 = -5.0; 
                    }
                }
                KeyCode::Right => {
                    if self.state == GameState::Play {
                        self.player_velocity.0 = 5.0;
                    }
                }
                KeyCode::Up => {
                    if self.state == GameState::Play  { //&& self.player_pos.1 == 460.0
                        self.player_velocity.1 = -10.0; 
                    }
                }
         
                _ => {}
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _: &mut Context, input: KeyInput)->GameResult {
        if let Some(keycode) = input.keycode {
            if self.state == GameState::Play {
                if keycode == KeyCode::Left || keycode == KeyCode::Right {
                    self.player_velocity.0 = 0.0;
                }
            }
        }
        Ok(())
    }
}

fn check_collision(grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT], x: f32, y: f32) -> bool {  
    let grid_x = (x / BLOCK_SIZE) as usize;
    let grid_y = (y / BLOCK_SIZE) as usize;

    if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
        grid[grid_y][grid_x]
    } else {
        false
    }
}

struct Enemy {
    pos: (f32, f32),
    velocity: (f32, f32),
}

impl Enemy {
    fn update(&mut self, grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {
        let next_x = self.pos.0 + self.velocity.0;
        let next_y = self.pos.1 + self.velocity.1;

        // Bewege den Gegner nur, wenn kein Hindernis im Weg ist
        if !check_collision(grid, next_x, self.pos.1) {
            self.pos.0 = next_x;
        } else {
            self.velocity.0 = -self.velocity.0; // Richtung umkehren
        }

        if !check_collision(grid, self.pos.0, next_y) {
            self.pos.1 = next_y;
        } else {
            self.velocity.1 = -self.velocity.1; // Richtung umkehren
        }
        if self.pos.0 < 0.0 || self.pos.0 > 800.0 {
            self.velocity.0 = -self.velocity.0; // Richtung umkehren
        }
        if self.pos.1 < 0.0 || self.pos.1 > 480.0 {
            self.velocity.1 = -self.velocity.1; // Richtung umkehren
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("bobble_clone", "author_name")
    .window_setup(ggez::conf::WindowSetup::default().title("bobble_clone"))
    .window_mode(WindowMode::default().dimensions(800.0, 480.0))
    .build()?;

    let game = Game::new();
    event::run(ctx, event_loop, game)
}



