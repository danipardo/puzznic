pub(crate) mod game_state;
pub mod levels;
pub(crate) mod sound;

use macroquad::prelude::*;

use self::sound::Mixer;

const TILE_WIDTH: f32 = 16f32;
const TILE_HEIGHT: f32 = 16f32;
const SPEED: f32 = 3.0;

#[derive(Debug)]
pub struct Tile {
    pub c: char,
    fade_step: u32,
    position: Vec2,
    velocity: Vec2,
    position_changed: bool,
    looping: bool,
    riding: bool,
    dragging_direction: Option<Direction>,
}

pub enum TileChange {
    Move,
    Bounce,
    Stop,
    FadeOut(usize),
    Copy(Tile),
    VelocityUpdate(Vec2),
    RidingFlag(bool),
}
impl Default for Tile {
    fn default() -> Self {
        Tile {
            c: ' ',
            fade_step: 0,
            position_changed: false,
            position: Vec2::new(0., 0.), // 0..15 relative to the tile coordinates in the map
            velocity: Vec2::new(0., 0.),
            looping: false,
            dragging_direction: None,
            riding: false,
        }
    }
}

impl Tile {
    pub fn blank() -> Tile {
        Tile::default()
    }
    pub fn new(c: char) -> Tile {
        let mut t = Tile::default();
        t.c = c;
        if c == '|' || c == '~' {
            t.riding = true;
        }
        return t;
    }

    pub fn is_playable(&self) -> bool {
        return self.c != ' ' && self.c != '-';
    }

    pub fn from(tile: &Tile) -> Tile {
        return Tile {
            position: tile.position,
            velocity: tile.velocity,
            c: tile.c,
            position_changed: tile.position_changed,
            looping: tile.looping,
            riding: tile.riding,
            fade_step: 0,
            dragging_direction: None,
        };
    }
    pub fn is_static(&self) -> bool {
        return self.c == ' ' || self.c == '-';
    }
}

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
pub fn handle_draw_player(level: &mut game_state::GameState) {
    // Draw player rectangle
    let (x, y) = (
        level.player.position.0 as f32 * TILE_WIDTH * 3.0,
        level.player.position.1 as f32 * TILE_WIDTH * 3.0,
    );

    draw_rectangle_lines(x, y, TILE_WIDTH * 3.0, TILE_HEIGHT * 3.0, 3., RED);
}

pub fn handle_draw_map(level: &mut game_state::GameState) -> bool {
    let dimensions = level.dimensions;
    let mut start_x = 0.0;
    let mut start_y = 0.0;

    let mut playable_pieces = 0;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let tile = level.get_tile_at(x, y);
            if tile.is_playable() {
                playable_pieces += 1;
            }
            if tile.c != ' ' {
                if tile.fade_step == 0 || tile.fade_step % 5 == 0 {
                    // draw_rectangle(
                    //     start_x,
                    //     start_y,
                    //     TILE_WIDTH * 3.,
                    //     TILE_HEIGHT * 3.,
                    //     WHITE,
                    // );

                    draw_texture_ex(
                        level.texture_map,
                        start_x + tile.position.x,
                        start_y + tile.position.y,
                        WHITE,
                        level.get_tile_texture_params(tile.c),
                    );
                }
            }
            start_x = start_x + TILE_HEIGHT * 3.0 as f32;
        }

        start_x = 0.0;
        start_y = start_y + TILE_WIDTH * 3.0;
    }

    playable_pieces == 0
}

pub fn handle_move_tiles(level: &mut game_state::GameState, _mixer: &mut Mixer) {
    let changes = level.next_map(&level.map);

    for change in &changes {
        let t = level
            .map
            .get_mut(change.1 * level.dimensions.0 + change.0)
            .unwrap();

        let tile_change = &change.2;
        match tile_change {
            TileChange::Stop => {
                println!("Stoping tile");
                t.velocity = Vec2::new(0., 0.);
            }
            TileChange::Move => {
                // println!(
                //     "Moving, I'm at {},{}, ({},{})",
                //     change.0, change.1, t.position.x, t.position.y
                // );
                t.position = t.position + t.velocity;
            }
            TileChange::Bounce => {
                t.velocity = t.velocity * -1.;
            }
            TileChange::FadeOut(_) => {
                t.fade_step = t.fade_step + 1;
                if t.fade_step >= 50 {
                    t.c = ' ';
                    t.fade_step = 0;
                }
            }
            TileChange::Copy(new_tile) => {
                t.position = new_tile.position;
                t.velocity = new_tile.velocity;
                t.c = new_tile.c;
                t.position_changed = new_tile.position_changed;
                t.looping = new_tile.looping;
                t.riding = new_tile.riding;
                t.dragging_direction = None;
            }
            TileChange::VelocityUpdate(vec2) => {
                t.velocity = *vec2;
            }
            TileChange::RidingFlag(flag) => {
                t.riding = *flag;
            }
        }
    }
}
pub fn handle_move_player(level: &mut game_state::GameState, mixer: &mut Mixer) -> bool {
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
pub async fn play_level(level: &mut game_state::GameState) {
    let camera = Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));

    let mut mixer = sound::Mixer::new();
    mixer.play_sound(sound::Sound::LevelIntro);

    loop {
        set_camera(camera);

        clear_background(GRAY);

        if handle_move_player(level, &mut mixer) {
            break;
        }
        handle_move_tiles(level, &mut mixer);

        if handle_draw_map(level) {
            println!("Level completed!");
            break;
        }
        handle_draw_player(level);

        next_frame().await;
    }
}
