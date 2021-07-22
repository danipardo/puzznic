pub(crate) mod game_logic;
pub mod levels;
pub(crate) mod sound;
pub mod tile;
use core::time;
use std::thread;

use macroquad::prelude::*;

use self::{sound::Mixer, tile::Tile, tile::TileChange};

const TILE_WIDTH: f32 = 16f32;
const TILE_HEIGHT: f32 = 16f32;
const SPEED: f32 = 1.;

pub struct Player {
    pub position: (usize, usize),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}
pub fn handle_draw_player(level: &mut game_logic::GameLogic) {
    // Draw player rectangle
    let (x, y) = (
        level.player.position.0 as f32 * TILE_WIDTH * 1.0,
        level.player.position.1 as f32 * TILE_WIDTH * 1.0,
    );

    draw_rectangle_lines(x, y, TILE_WIDTH *1.0, TILE_HEIGHT * 1.0, 2., RED);
}

pub fn handle_draw_map(level: &mut game_logic::GameLogic) -> bool {
    //  let dimensions = level.dimensions;
    // let mut start_x = 0.0;
    // let mut start_y = 0.0;

    let mut playable_pieces = 0;
    //    for y in 0..dimensions.1 {
    //      for x in 0..dimensions.0 {
    for tile in &level.map {
        // let tile = level.get_tile_at(x, y);
        if tile.is_playable() {
            playable_pieces += 1;
        }
            if tile.fade_step % 4 == 0 {
           
                draw_texture_ex(
                    level.texture_map,
                    tile.position.x,
                    tile.position.y,
                    WHITE,
                    level.get_tile_texture_params(tile.c),
                );
            }
    }


    playable_pieces == 0
}

pub fn handle_move_tiles(level: &mut game_logic::GameLogic, _mixer: &mut Mixer) {
    let changes = level.next_map(&level.map);
    let mut drain: Vec<u32> = vec![];
    level.fading_out = false;
    for (index,  tile_change) in &changes {
        let t = level.map.get_mut(*index).unwrap();
        match tile_change {
            TileChange::Stop => {
                t.velocity = Vec2::zero();
            }
            TileChange::Move => {
                // println!(
                //     "Moving, I'm at {},{}, ({},{})",
                //     change.0, change.1, t.position.x, t.position.y
                // );
                t.position = t.position + t.velocity;
            },
            TileChange::Jump(position) => {
                t.position = *position;
                t.velocity = Vec2::zero();
                t.dragging_direction = None;
            },
            TileChange::Bounce => {
                t.velocity = t.velocity * -1.;
            }
            TileChange::FadeOut => {
                t.fade_step = t.fade_step + 1;
                level.fading_out = true;
                if t.fade_step >= 50 {
                   drain.push(t.id);
                }
            }
            TileChange::StartRiding(velocity) => {
                t.riding = true;
                t.velocity = *velocity;
            }
            TileChange::Fall => {
                t.velocity = Vec2::new(0., SPEED);
                t.riding = false;
            }
            
            TileChange::VelocityUpdate(vec2) => {
                t.velocity = *vec2;
            }
            TileChange::RidingFlag(flag) => {
                t.riding = *flag;
            }
        }
    }
    level.map.retain(|e| !drain.contains(&e.id));

}
pub fn handle_move_player(level: &mut game_logic::GameLogic, mixer: &mut Mixer) -> bool {
    if is_key_down(KeyCode::Escape) {
        return true;
    }

    if is_key_pressed(KeyCode::Left) {
        level.move_player(Direction::Left, mixer);
    }
    if is_key_pressed(KeyCode::Right) {
        level.move_player(Direction::Right, mixer);
    }

    if is_key_pressed(KeyCode::Up) && !level.dragging {
        level.move_player(Direction::Up, mixer);
    }

    if is_key_down(KeyCode::Space) {
        level.dragging = true;
    } else {
        level.dragging = false;
    }

    if is_key_pressed(KeyCode::Down) {
        level.move_player(Direction::Down, mixer);
    }
    if is_key_pressed(KeyCode::Escape) {
        return true;
    }

    false
}
pub async fn play_level(level: &mut game_logic::GameLogic) {
    //    let camera = Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));
    let camera = Camera2D::from_display_rect(Rect::new(0., 0., 320., 200.));

    let mut mixer = sound::Mixer::new();
    //  mixer.play_sound(sound::Sound::LevelIntro);

    loop {

        clear_background(GRAY);
        set_camera(camera);
        if handle_move_player(level, &mut mixer) {
            break;
        }
        if !is_key_down(KeyCode::Space) {
            handle_move_tiles(level, &mut mixer);
        }

        // let ten_millis = time::Duration::from_millis(30);
        //  thread::sleep(ten_millis);

        if handle_draw_map(level) {
            println!("Level completed!");
            break;
        }
        handle_draw_player(level);

        next_frame().await;
    }
}
