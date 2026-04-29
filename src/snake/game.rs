use crate::menu::Application;
use crate::snake::game::vec::Vec;
use crate::system::vga::palette;
use crate::system;
use alloc::vec;
use core::cmp::PartialEq;
use spin::Mutex;
use crate::system::random;

pub struct SnakeGame;

impl Application for SnakeGame {
    fn name(&self) -> &'static str {
        "Snake"
    }

    fn run(&self) {
        init();
    }
}

struct GameState {
    grid: [[Tile; 20]; 32],
    head_location: (i8, i8),
    apple_location: (i8, i8),
    turn_points: Vec<(i8, i8, Direction)>,
    current_length: usize,
    current_direction: Direction,
    game_over: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum GameOverChoice {
    PlayAgain,
    ReturnToMenu,
}


#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_previous_tile(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (1, 0),
            Direction::Right => (-1, 0),
        }
    }

    pub fn get_next_tile(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn is_opposite(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down) |
            (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) |
            (Direction::Right, Direction::Left)
        )
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Snake,
    Border,
    Apple,
    Empty,
}

impl Tile {
    pub fn draw(&self, x: usize, y: usize) {
        system::vga::draw_rect(x * 10, y * 10, 10, 10, self.get_color());
    }

    pub fn get_color(&self) -> u8 {
        match self {
            Tile::Snake => palette::GREEN,
            Tile::Border => palette::CYAN,
            Tile::Apple => palette::RED,
            Tile::Empty => palette::BLACK,
        }
    }
}

static GAME_STATE: Mutex<Option<GameState>> = Mutex::new(None);
static LATEST_INPUT: Mutex<Direction> = Mutex::new(Direction::Right);
static EXIT: Mutex<bool> = Mutex::new(false);
static GAME_OVER_CHOICE: Mutex<Option<GameOverChoice>> = Mutex::new(None);

pub fn init() {
    system::vga::set_mode_320x200x256();
    random::seed_from_timer();

    loop {
        start_game();

        let choice = *GAME_OVER_CHOICE.lock();
        match choice {
            Some(GameOverChoice::PlayAgain) => {
                *GAME_OVER_CHOICE.lock() = None;
                continue;
            }
            Some(GameOverChoice::ReturnToMenu) | None => {
                break;
            }
        }
    }

    system::vga::set_text_mode_80x25();
}

fn start_game() {
    use crate::system::keyboard;
    use pc_keyboard::DecodedKey;

    system::vga::clear_screen_pixel(palette::BLACK);

    *GAME_STATE.lock() = Some(GameState {
        grid: [[Tile::Empty; 20]; 32],
        head_location: (16, 10),
        apple_location: (0, 0),
        turn_points: vec!(),
        current_length: 3,
        current_direction: Direction::Right,
        game_over: false,
    });

    {
        let mut guard = GAME_STATE.lock();
        let game_state = guard.as_mut().unwrap();
        generate_apple(game_state);
    }

    *EXIT.lock() = false;
    *LATEST_INPUT.lock() = Direction::Right;

    keyboard::set_key_handler(|key: DecodedKey| {
        match key {
            DecodedKey::RawKey(pc_keyboard::KeyCode::Escape) => {
                *EXIT.lock() = true;
            },
            DecodedKey::Unicode('w') => *LATEST_INPUT.lock() = Direction::Up,
            DecodedKey::Unicode('s') => *LATEST_INPUT.lock() = Direction::Down,
            DecodedKey::Unicode('a') => *LATEST_INPUT.lock() = Direction::Left,
            DecodedKey::Unicode('d') => *LATEST_INPUT.lock() = Direction::Right,
            _ => {}
        }
    });

    system::timer::reset_tick_count();
    system::timer::set_timer_handler(game_tick);

    x86_64::instructions::interrupts::enable();

    loop {
        let is_game_over = {
            let guard = GAME_STATE.lock();
            guard.as_ref().map(|s| s.game_over).unwrap_or(false)
        };

        if is_game_over {
            let score = {
                let guard = GAME_STATE.lock();
                guard.as_ref().map(|s| s.current_length.saturating_sub(3)).unwrap_or(0)
            };

            *GAME_OVER_CHOICE.lock() = None;
            draw_game_over_screen(score);
            keyboard::set_key_handler(game_over_key_handler);

            loop {
                if GAME_OVER_CHOICE.lock().is_some() {
                    break;
                }
                x86_64::instructions::interrupts::enable_and_hlt();
            }
            break;
        }

        if *EXIT.lock() {
            *GAME_OVER_CHOICE.lock() = Some(GameOverChoice::ReturnToMenu);
            break;
        }

        x86_64::instructions::interrupts::enable_and_hlt();
    }

    system::timer::clear_timer_handler();
    keyboard::clear_key_handler();
    *GAME_STATE.lock() = None;
}

fn game_tick() {
    let mut guard = GAME_STATE.lock();
    if guard.is_none() {
        return;
    }

    let ticks = system::timer::get_tick_count();
    if ticks % 3 == 0 {
        if let Some(game_state) = guard.as_mut() {
            if !game_state.game_over {
                draw_frame(game_state);
            }
        }
    }
}

fn draw_frame(game_state: &mut GameState) {
    game_state.grid = [[Tile::Empty; 20]; 32];

    position_border(game_state);
    position_apple(game_state);
    position_snake(game_state);
    hande_input(game_state);
    advance_head(game_state);

    for i in 0..32 {
        for j in 0..20 {
            let tile = game_state.grid[i][j];
            tile.draw(i, j);
        }
    }
}

fn position_border(game_state: &mut GameState) {
    for i in 0..32 {
        for j in 0..20 {
            if i == 0 || i == 31 || j == 0 || j == 19 {
                game_state.grid[i][j] = Tile::Border;
            }
        }
    }
}

fn position_apple(game_state: &mut GameState) {
    let apple = game_state.apple_location;
    game_state.grid[apple.0 as usize][apple.1 as usize] = Tile::Apple;
}

fn get_snake_segments(game_state: &GameState) -> Vec<(i8, i8)> {
    let mut segments = vec![game_state.head_location];
    let mut last_segment = game_state.head_location;
    let mut current_traversal_direction = game_state.current_direction.clone();

    let body_segments = game_state.current_length.saturating_sub(1);

    for _ in 0..body_segments {
        for tp in &game_state.turn_points {
            if tp.0 == last_segment.0 && tp.1 == last_segment.1 {
                current_traversal_direction = tp.2.clone();
                break;
            }
        }

        let previous = current_traversal_direction.get_previous_tile();
        let proposed_location = (last_segment.0 + previous.0, last_segment.1 + previous.1);
        segments.push(proposed_location);
        last_segment = proposed_location;
    }

    segments
}

fn position_snake(game_state: &mut GameState) {
    let segments = get_snake_segments(game_state);

    for (i, segment) in segments.iter().enumerate() {
        game_state.grid[segment.0 as usize][segment.1 as usize] = Tile::Snake;

        if i == segments.len() - 1 {
            game_state.turn_points.retain(|tp| tp.0 != segment.0 || tp.1 != segment.1);
        }
    }
}

fn advance_head(game_state: &mut GameState) {
    let direction = &game_state.current_direction;

    game_state.head_location.0 += direction.get_next_tile().0;
    game_state.head_location.1 += direction.get_next_tile().1;

    let new_tile = game_state.grid[game_state.head_location.0 as usize][game_state.head_location.1 as usize];
    match new_tile {
        Tile::Snake => game_state.game_over = true,
        Tile::Border => game_state.game_over = true,
        Tile::Apple => {
            game_state.current_length += 1;
            generate_apple(game_state);
        }
        _ => {}
    }
}

fn hande_input(game_state: &mut GameState) {
    let guard = LATEST_INPUT.lock();
    let input_direction = guard.clone();

    if input_direction != game_state.current_direction && !game_state.current_direction.is_opposite(&input_direction) {
        let old_direction = game_state.current_direction.clone();
        game_state.current_direction = input_direction.clone();

        let head = game_state.head_location;
        game_state.turn_points.push((head.0, head.1, old_direction));
    }
}

fn generate_apple(game_state: &mut GameState) {
    let segments = get_snake_segments(game_state);

    loop {
        let x = random::next_range(1, 31) as i8;
        let y = random::next_range(1, 19) as i8;

        let occupied = segments.iter().any(|seg| seg.0 == x && seg.1 == y);

        if !occupied {
            game_state.apple_location = (x, y);
            break;
        }
    }
}

fn draw_pixel_text(text: &str, x: usize, y: usize, color: u8) {
    const FONT: [[u8; 8]; 128] = include!("font_data.txt");

    let mut offset = 0;
    for ch in text.chars() {
        let ch_code = ch as usize;
        if ch_code < 128 {
            let glyph = FONT[ch_code];
            for row in 0..8 {
                let bits = glyph[row];
                for col in 0..8 {
                    if (bits >> col) & 1 == 1 {
                        system::vga::put_pixel(x + offset + col, y + row, color);
                    }
                }
            }
        }
        offset += 9;
    }
}

fn draw_game_over_screen(score: usize) {
    system::vga::clear_screen_pixel(palette::BLACK);

    system::vga::draw_rect(40, 40, 240, 120, palette::DARK_GRAY);
    system::vga::draw_rect(42, 42, 236, 116, palette::BLACK);

    draw_pixel_text("GAME OVER", 110, 60, palette::RED);

    let score_text = alloc::format!("Score: {}", score);
    draw_pixel_text(&score_text, 100, 80, palette::YELLOW);

    draw_pixel_text("R - Play Again", 85, 110, palette::WHITE);
    draw_pixel_text("M - Return to Menu", 70, 130, palette::WHITE);
}

fn game_over_key_handler(key: pc_keyboard::DecodedKey) {
    use pc_keyboard::DecodedKey;

    match key {
        DecodedKey::Unicode('r') | DecodedKey::Unicode('R') => {
            *GAME_OVER_CHOICE.lock() = Some(GameOverChoice::PlayAgain);
        }
        DecodedKey::Unicode('m') | DecodedKey::Unicode('M') => {
            *GAME_OVER_CHOICE.lock() = Some(GameOverChoice::ReturnToMenu);
        }
        DecodedKey::RawKey(pc_keyboard::KeyCode::Escape) => {
            *GAME_OVER_CHOICE.lock() = Some(GameOverChoice::ReturnToMenu);
        }
        _ => {}
    }
}