use async_trait::async_trait;
use macroquad::prelude::*;

use crate::game::tile::TileChange;

use super::{
    game_logic::{self, PlayingState},
    menu_state::MenuState,
    sound::{self, Mixer},
    states::{Playable, StateType},
};

// use self::{sound::Mixer, tile::Tile, tile::TileChange};

pub const TILE_WIDTH: f32 = 16f32;
pub const TILE_HEIGHT: f32 = 16f32;
pub const SPEED: f32 = 1.;

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
pub fn handle_draw_player(level: &mut PlayingState) {
    // Draw player rectangle
    let (x, y) = (
        level.player.position.0 as f32 * TILE_WIDTH * 1.0,
        level.player.position.1 as f32 * TILE_WIDTH * 1.0,
    );

    draw_rectangle_lines(x, y, TILE_WIDTH * 1.0, TILE_HEIGHT * 1.0, 2., RED);
}

pub fn handle_draw_map(level: &mut PlayingState) -> bool {
    // draw a grey background

   //  let (offset_x, offset_y) = (120., 5.);

    for tile in &level.blanks {
        macroquad::shapes::draw_rectangle(
            tile.position.x + level.offset_x,
            tile.position.y + level.offset_y,
            TILE_HEIGHT,
            TILE_HEIGHT,
            BLACK,
        );
    }
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
                tile.position.x + level.offset_x,
                tile.position.y + level.offset_y,
                WHITE,
                level.get_tile_texture_params(tile.c),
            );
        }
    }

    playable_pieces == 0
}

pub fn handle_move_tiles(level: &mut PlayingState, _mixer: &mut Mixer) {
    let changes = level.next_map(&level.map);
    let mut drain: Vec<u32> = vec![];
    level.fading_out = false;
    for (index, tile_change) in &changes {
        let t = level.map.get_mut(*index).unwrap();
        match tile_change {
            TileChange::Stop => {
                t.velocity = Vec2::zero();
                t.dragging_direction = None;
            }
            TileChange::Move => {
                // println!(
                //     "Moving, I'm at {},{}, ({},{})",
                //     change.0, change.1, t.position.x, t.position.y
                // );
                t.position = t.position + t.velocity;
            }
            TileChange::Jump(position) => {
                t.position = *position;
                t.velocity = Vec2::zero();
                t.dragging_direction = None;
            }
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
pub fn handle_move_player(level: &mut PlayingState, mixer: &mut Mixer) {
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
}

pub fn draw_score(level: &mut PlayingState) {
    draw_texture(level.scoreboard_texture, 0., 0., WHITE);
    let (fs, fc, fa) = camera_font_scale(6.);
    let tp = TextParams {
        font: level.font,
        font_size: fs,
        font_scale: fc,
        font_scale_aspect: fa,
        color: GREEN,
    };

    draw_text_ex("SCORE: 0", 10., 13., tp);

    for y in 0..25 {
        let mut offset = 0.;
        if y % 2 > 0 {
            offset += 8.;
        }
        for x in 0..13 {
            draw_texture(
                level.brick_decoration,
                101. + offset + (x * 16) as f32,
                2. + (y * 8) as f32,
                WHITE,
            )
        }
    }
}
#[async_trait]
impl Playable for PlayingState {
    async fn run(&mut self) -> super::states::StateType {
        let desired_ratio = 320. / 200. as f32;

        let mut mixer = sound::Mixer::new();
        //  mixer.play_sound(sound::Sound::LevelIntro);

        loop {
            let physical_ratio = screen_width() / screen_height();
            // println!(
            //     "Disp.Ratio: {}, Other: {}",
            //     physical_ratio,
            //     (physical_ratio / &desired_ratio)
            // );
            let mut width_factor = 1.;
            let mut height_factor = 1.;

            if physical_ratio / desired_ratio > 1. {
                width_factor = physical_ratio / desired_ratio;
            } else {
                height_factor = physical_ratio / desired_ratio;
            }
            let camera = Camera2D::from_display_rect(Rect::new(
                0.,
                0.,
                320. * width_factor,
                200. / height_factor,
            ));
            clear_background(BLACK);
            draw_score(&mut self);
            set_camera(&camera);
            handle_move_player(self, &mut mixer);
            if !is_key_down(KeyCode::Space) {
                handle_move_tiles(self, &mut mixer);
            }

            // let ten_millis = time::Duration::from_millis(30);
            //  thread::sleep(ten_millis);

            if handle_draw_map(self) {
                println!("Level completed!");
                break;
            }
            handle_draw_player(self);

            next_frame().await;
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
        }

        StateType::Menu
    }
}
