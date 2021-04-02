pub(crate) mod game_state;

use std::{collections::vec_deque, rc::Rc};

use game_state::GameState;
use macroquad::prelude::*;

const TILE_WIDTH: f32 = 16f32;
const TILE_HEIGHT: f32 = 16f32;
const SPEED: f32 = 2.0;

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

    pub fn handle_collision(&mut self, neighbors: Vec<&Tile>) {}
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
pub fn handle_draw_player(level: &mut game_state::GameState) {
    // Draw player rectangle
    let (x, y) = (
        level.player.position.0 as f32 * TILE_WIDTH * 3.0,
        level.player.position.1 as f32 * TILE_WIDTH * 3.0,
    );

    draw_rectangle_lines(x, y, TILE_WIDTH * 3.0, TILE_HEIGHT * 3.0, 3., RED);
}

pub fn handle_draw_map(level: &mut game_state::GameState) {
    let dimensions = level.dimensions;
    let mut start_x = 0.0;
    let mut start_y = 0.0;

    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let tile = level.get_tile_at(x, y);
            if tile.c != 'x' {
                draw_texture_ex(
                    level.texture_map,
                    start_x + tile.position.x,
                    start_y + tile.position.y,
                    WHITE,
                      level.get_tile_texture_params(tile.c),
                );
            }
            start_x = start_x + TILE_HEIGHT * 3.0 as f32;
        }

        start_x = 0.0;
        start_y = start_y + TILE_WIDTH * 3.0;
    }
}

pub fn handle_move_tiles(level: &mut game_state::GameState) {
  

    let changes = level.next_map(&level.map);

    for change in &changes {
        let t = level
            .map
            .get_mut(change.1 * level.dimensions.0 + change.0)
            .unwrap();

        t.position = change.2.position;
        t.velocity = change.2.velocity;
    
    }
}
pub fn handle_move_player(level: &mut game_state::GameState) -> bool {
    if is_key_down(KeyCode::Escape) {
        return true;
    }

    if is_key_pressed(KeyCode::Left) {
        level.move_player(Direction::Left);
    }
    if is_key_pressed(KeyCode::Right) {
        level.move_player(Direction::Right);
    }

    if is_key_pressed(KeyCode::Up) && !level.dragging {
        level.move_player(Direction::Up);
    }

    if is_key_down(KeyCode::Space) {
        level.dragging = true;
    } else {
        level.dragging = false;
    }

    if is_key_pressed(KeyCode::Down) {
        level.move_player(Direction::Down);
    }

    false
}
pub async fn play_level(level: &mut game_state::GameState) {
    let camera = Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));

    loop {
        debug!("Screen: {},{}", screen_width(), screen_height());
        set_camera(camera);

        clear_background(GRAY);

        handle_move_player(level);
        handle_move_tiles(level);

        handle_draw_map(level);
        handle_draw_player(level);

        next_frame().await;
    }
}
