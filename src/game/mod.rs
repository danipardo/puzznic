pub(crate) mod GameState;

use macroquad::*;

const TILE_WIDTH: f32 = 48f32;
const TILE_HEIGHT: f32 = 48f32;
const SPEED: f32 = 0.2;

#[derive(Debug)]
pub struct Tile {
    pub c: char,
    // slide_step: u32,
    position: Vec2,
    velocity: Vec2,
    position_changed: bool,
    looping: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            c: 'x',
            // slide_step: 0,
            position_changed: false,
            position: Vec2::new(0., 0.),
            velocity: Vec2::new(0., 0.),
            looping: false,
        }
    }
}

impl Tile {
    pub fn blank() -> Tile {
        Tile::default()
    }

    pub fn do_move(&mut self) -> bool {
        if self.velocity == Vec2::zero() {
            return false;
        }
        self.position = self.position + self.velocity;

        debug!("Tile position: {}", self.position);
        false
    }
}

pub struct Player {
    pub position: (usize, usize),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}
pub fn handle_draw_player(level: &mut GameState::GameState) {
    // Draw player rectangle
    let (x, y) = level.tile_to_coords(level.player.position.0, level.player.position.1);

    draw_rectangle_lines(x, y, TILE_WIDTH, TILE_HEIGHT, 8., RED);
}

pub fn handle_draw_map(level: &mut GameState::GameState) {
    let dimensions = level.dimensions;
    let mut start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
    let mut start_y = screen_height() / 2. - dimensions.1 as f32 * TILE_HEIGHT / 2. as f32;

    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let tile = level.get_tile_at(x, y);
            if tile.c != 'x' {
                draw_texture_ex(
                    level.texture_map,
                    start_x + tile.position.x(),
                    start_y + tile.position.y(),
                    WHITE,
                    level.get_tile_texture_params(tile.c),
                );
            }
            start_x = start_x + TILE_HEIGHT as f32;
        }

        start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
        start_y = start_y + TILE_WIDTH;
    }
}

pub fn handle_move_tiles(level: &mut GameState::GameState) {
    for y in 0..level.dimensions.0 {
        for x in 0..level.dimensions.1 {
            let tile = level.map.get_mut(y * level.dimensions.0 + x).unwrap();
            let _changed_cell = tile.do_move();
        }
    }

    let changes = level.next_map(&level.map);

    for change in &changes {
        let t = level
            .map
            .get_mut(change.1 * level.dimensions.0 + change.0)
            .unwrap();

        t.position = change.2.position;
        t.velocity = change.2.velocity;
        t.c = change.2.c;
        // t.slide_step = change.2.slide_step;
        t.looping = change.2.looping;
        t.position_changed = change.2.position_changed;
    }
    if changes.len() > 0 {
        //  self.draw_map(&self.map);
    }
}
pub fn handle_move_player(level: &mut GameState::GameState) -> bool {
    if macroquad::is_key_down(KeyCode::Escape) {
        return true;
    }

    if macroquad::is_key_pressed(KeyCode::Left) {
        level.move_player(Direction::Left);
    }
    if macroquad::is_key_pressed(KeyCode::Right) {
        level.move_player(Direction::Right);
    }

    if macroquad::is_key_pressed(KeyCode::Up) && !level.dragging {
        level.move_player(Direction::Up);
    }

    if macroquad::is_key_down(KeyCode::Space) {
        level.dragging = true;
    } else {
        level.dragging = false;
    }

    if macroquad::is_key_pressed(KeyCode::Down) {
        level.move_player(Direction::Down);
    }

    false
}
pub async fn play_level(level: &mut GameState::GameState) {
    loop {
        clear_background(GRAY);


        handle_move_player(level);
        handle_move_tiles(level);

        handle_draw_map(level);
        handle_draw_player(level);

        next_frame().await;
    }
}
