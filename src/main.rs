use macroquad::prelude::*;

// Screen dimension constants
pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 720;
pub const SPEED: i32 = 4;
pub const PLAYER_WIDTH: i32 = 3;
pub const MAX_PLAYER: usize = 4;


// Color aliases (matching original intent)
pub const WHITE_COLOR: Color = WHITE;
pub const BLACK_COLOR: Color = BLACK;
pub const BLUE_COLOR: Color = BLUE;
pub const RED_COLOR: Color = RED;
pub const GREEN_COLOR: Color = GREEN;
pub const YELLOW_COLOR: Color = YELLOW;


// Struct for Lightcycle Player
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub dir: i32,   // 0: down, 1: right, 2: up, 3: left
    pub active: bool,
    pub color: Color,
}

impl Player {
    pub fn new(color: Color, screen_width : i32, screen_height : i32) -> Self {
        use macroquad::rand::gen_range;
        let x = gen_range(0, screen_width);
        let y = gen_range(0, screen_height);
        let dir = gen_range(0, 4);
        Self { x, y, dir, active: true, color }
    }

    pub fn tick(&mut self, screen_width: i32, screen_height: i32) {
        // Move according to direction
        match self.dir {
            0 => self.y += 1,
            1 => self.x += 1,
            2 => self.y -= 1,
            3 => self.x -= 1,
            _ => {}
        }

        // Wrap around screen bounds
        if self.x >= screen_width { self.x = 0; }
        if self.x < 0 { self.x = screen_width - 1; }
        if self.y >= screen_height { self.y = 0; }
        if self.y < 0 { self.y = screen_height - 1; }
    }

    pub fn reset(&mut self, screen_width: i32, screen_height: i32) {
        use macroquad::rand::gen_range;
        self.x = gen_range(0, screen_width);
        self.y = gen_range(0, screen_height);
        self.dir = gen_range(0, 4);
        self.active = true;
    }
}

// Change player directions
fn update_player_dir(player: &mut Player, key_left: KeyCode, key_right: KeyCode) {
    if is_key_pressed(key_left) {
        player.dir = (player.dir + 1) % 4;
    }
    if is_key_pressed(key_right) {
        player.dir = (player.dir + 3) % 4;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "lightCycles".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Rust/macroquad implementation converted from the original C main

    let screen_height = screen_height().round() as i32;
    let screen_width = screen_width().round() as i32;

    let mut players = [
        Player::new(RED_COLOR, screen_width, screen_height),
        Player::new(GREEN_COLOR, screen_width, screen_height),
        Player::new(BLUE_COLOR, screen_width, screen_height),
        Player::new(YELLOW_COLOR, screen_width, screen_height),
    ];

    let mut player_count: usize = 4;
    let mut game = true;
    let mut reset = false;
    let mut victory_displayed = false;
    let mut winner: Option<usize> = None;

    // Occupied cells and a list of points to redraw trails
    let mut field = vec![vec![false; screen_height as usize]; screen_width as usize];
    let mut trail_points: Vec<(i32, i32, Color)> = Vec::new();

    // Initial clear background
    clear_background(BLACK_COLOR);

    loop {
        // Input handling (direction changes and controls)
        use macroquad::input::KeyCode;

        // Player 1 LEFT/RIGHT (X/C)
        update_player_dir(&mut players[0], KeyCode::X, KeyCode::C);

        // Player 2 LEFT/RIGHT (Left/Right arrows)
        update_player_dir(&mut players[0], KeyCode::Left, KeyCode::Right);

        // Player 3 LEFT/RIGHT (A/Q)
        update_player_dir(&mut players[0], KeyCode::A, KeyCode::Q);

        // Player 4 LEFT/RIGHT (use top-row 6/9 keys as a substitute)
        update_player_dir(&mut players[3], KeyCode::Key6, KeyCode::Key9);

        // Global controls
        if is_key_pressed(KeyCode::Escape) {
            break; // quit
        }
        if is_key_pressed(KeyCode::R) {
            reset = true; // request reset
        }
        if is_key_pressed(KeyCode::F1) { player_count = 1; }
        if is_key_pressed(KeyCode::F2) { player_count = 2; }
        if is_key_pressed(KeyCode::F3) { player_count = 3; }
        if is_key_pressed(KeyCode::F4) { player_count = 4; }

        // reset game when needed
        if reset {
            // Drawing
            clear_background(BLACK_COLOR);
            game = true;
            for col in field.iter_mut() { for cell in col.iter_mut() { *cell = false; } }
            trail_points.clear();
            for p in 0..MAX_PLAYER { players[p].reset(screen_width, screen_height); }
            victory_displayed = false;
            winner = None;
            reset = false;
        }

        // Update game state
        if game {
            for _ in 0..SPEED {
                for u in 0..player_count {
                    if players[u].active { players[u].tick(screen_width, screen_height); }
                }

                let mut alive_players = 0usize;
                for u in 0..player_count {
                    let x = players[u].x as usize;
                    let y = players[u].y as usize;
                    if field[x][y] { players[u].active = false; }
                    if players[u].active { alive_players += 1; }
                }
                if alive_players <= 1 { game = false; }

                for u in 0..player_count {
                    if players[u].active {
                        let x = players[u].x as usize;
                        let y = players[u].y as usize;
                        if !field[x][y] {
                            trail_points.push((players[u].x, players[u].y, players[u].color));
                        }
                        field[x][y] = true;
                    }
                }
            }
        } else if !victory_displayed {
            // Determine winner
            winner = (0..player_count).find(|&u| players[u].active);
            victory_displayed = true;
        }

        // Draw trails
        for &(x, y, color) in &trail_points {
            draw_rectangle(x as f32, y as f32, PLAYER_WIDTH as f32, PLAYER_WIDTH as f32, color);
        }

        // Draw current player positions on top
        for u in 0..player_count {
            if players[u].active {
                draw_rectangle(players[u].x as f32, players[u].y as f32, PLAYER_WIDTH as f32, PLAYER_WIDTH as f32, players[u].color);
            }
        }

        // If game ended, show winner text
        if !game {
            let msg = match winner {
                Some(w) => format!("Player {} wins", w + 1),
                None => "Draw".to_string(),
            };
            let text_size = 40.0;
            let text_dim = measure_text(&msg, None, text_size as u16, 1.0);
            draw_text(
                &msg,
                (screen_width as f32 - text_dim.width) / 2.0,
                (screen_height as f32 - text_dim.height) / 2.0,
                text_size,
                WHITE_COLOR,
            );
        }

        next_frame().await;
    }
}
