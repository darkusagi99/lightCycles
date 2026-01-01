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


// Struct for TRON Player
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub dir: i32,   // 0: down, 1: right, 2: up, 3: left
    pub active: bool,
    pub color: Color,
}

impl Player {
    pub fn new(color: Color) -> Self {
        use macroquad::rand::gen_range;
        let x = gen_range(0, SCREEN_WIDTH.max(1));
        let y = gen_range(0, SCREEN_HEIGHT.max(1));
        let dir = gen_range(0, 4);
        Self { x, y, dir, active: true, color }
    }

    pub fn tick(&mut self) {
        // Move according to direction
        match self.dir {
            0 => self.y += 1,
            1 => self.x += 1,
            2 => self.y -= 1,
            3 => self.x -= 1,
            _ => {}
        }

        // Wrap around screen bounds
        if self.x >= SCREEN_WIDTH { self.x = 0; }
        if self.x < 0 { self.x = SCREEN_WIDTH - 1; }
        if self.y >= SCREEN_HEIGHT { self.y = 0; }
        if self.y < 0 { self.y = SCREEN_HEIGHT - 1; }
    }

    pub fn reset(&mut self) {
        use macroquad::rand::gen_range;
        self.x = gen_range(0, SCREEN_WIDTH.max(1));
        self.y = gen_range(0, SCREEN_HEIGHT.max(1));
        self.dir = gen_range(0, 4);
        self.active = true;
    }
}

// Toggle fullscreen helper analogous to the original ToggleFullscreen
pub fn toggle_fullscreen() {
    use macroquad::window::set_fullscreen;
    // Minimal implementation: request fullscreen
    set_fullscreen(true);
}


#[macroquad::main("lightCycles")]
async fn main() {
    // Rust/macroquad implementation converted from the original C main
    let mut players = [
        Player::new(RED_COLOR),
        Player::new(GREEN_COLOR),
        Player::new(BLUE_COLOR),
        Player::new(YELLOW_COLOR),
    ];

    let mut player_count: usize = 4;
    let mut game = true;
    let mut reset = false;
    let mut victory_displayed = false;
    let mut winner: Option<usize> = None;

    // Occupied cells and a list of points to redraw trails
    let mut field = vec![vec![false; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];
    let mut trail_points: Vec<(i32, i32, Color)> = Vec::new();

    loop {
        // Input handling (direction changes and controls)
        use macroquad::input::KeyCode;

        // Player 1 LEFT/RIGHT (X/C)
        if is_key_pressed(KeyCode::X) {
            players[0].dir = (players[0].dir + 1) % 4;
        }
        if is_key_pressed(KeyCode::C) {
            players[0].dir = (players[0].dir + 3) % 4;
        }

        // Player 2 LEFT/RIGHT (Left/Right arrows)
        if is_key_pressed(KeyCode::Left) {
            players[1].dir = (players[1].dir + 1) % 4;
        }
        if is_key_pressed(KeyCode::Right) {
            players[1].dir = (players[1].dir + 3) % 4;
        }

        // Player 3 LEFT/RIGHT (A/Q)
        if is_key_pressed(KeyCode::A) {
            players[2].dir = (players[2].dir + 1) % 4;
        }
        if is_key_pressed(KeyCode::Q) {
            players[2].dir = (players[2].dir + 3) % 4;
        }

        // Player 4 LEFT/RIGHT (use top-row 6/9 keys as a substitute)
        if is_key_pressed(KeyCode::Key6) {
            players[3].dir = (players[3].dir + 1) % 4;
        }
        if is_key_pressed(KeyCode::Key9) {
            players[3].dir = (players[3].dir + 3) % 4;
        }

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
            game = true;
            for col in field.iter_mut() { for cell in col.iter_mut() { *cell = false; } }
            trail_points.clear();
            for p in 0..MAX_PLAYER { players[p].reset(); }
            victory_displayed = false;
            winner = None;
            reset = false;
        }

        // Update game state
        if game {
            for _ in 0..SPEED {
                for u in 0..player_count {
                    if players[u].active { players[u].tick(); }
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

        // Drawing
        clear_background(BLACK_COLOR);

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
                (SCREEN_WIDTH as f32 - text_dim.width) / 2.0,
                (SCREEN_HEIGHT as f32 - text_dim.height) / 2.0,
                text_size,
                WHITE_COLOR,
            );
        }

        next_frame().await;
    }
}
