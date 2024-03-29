use super::{playing_state::*, sound::{Mixer, self}, tile::Tile};
use crate::game::tile::TileChange;
use macroquad::prelude::*;
use std::{collections::HashMap};

pub struct PlayingState {
    pub map: Vec<Tile>,
    pub blanks: Vec<Tile>,      // empty spaces to draw the background
    pub texture_map: Texture2D, // single image that contains all the tiles
    pub scoreboard_texture: Texture2D,
    pub brick_decoration: Texture2D,
    pub tile_info: HashMap<char, u32>, // image offset of each tile in the main image
    pub dimensions: (usize, usize),    // map dimensions,
    pub level: usize,
    pub time: u16,
    pub paused: bool,
    pub exit_intent: bool,
    pub offset_x: f32,
    pub offset_y: f32,
    pub font: Font,
    pub player: Player,
    pub score: u32,
    pub time_elpsed: u32,
    pub fading_out: bool,
    pub dragging: bool,
    pub dragging_step: u8
}

pub struct LevelInfo {
    pub tiles: Vec<Tile>,
    pub blanks: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    pub offset_x: f32,
    pub offset_y: f32,
    pub level: usize,
    pub time: u16
}

/// AABB collision detection, returns true if collision found
fn check_collision_perfect(t1: &Tile, coordinates: &Vec2) -> bool {
     (t1.position.x - coordinates.x).abs() < TILE_WIDTH
        && (t1.position.y - coordinates.y).abs() < TILE_HEIGHT
}

impl PlayingState {
    pub fn get_tile_texture_params(&self, c: char) -> DrawTextureParams {
        let offset = *self
            .tile_info
            .get(&c)
            .expect(format!("cannot find tile {}", c).as_str()) as f32;
        // let ratio = screen_width() / screen_height();
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(TILE_WIDTH * 1., TILE_HEIGHT * 1. as f32)),
            source: Some(Rect::new(offset, 0., TILE_WIDTH as f32, TILE_HEIGHT as f32)),
            rotation: 0.,
            pivot: None,
            flip_x: false,
            flip_y: false,
        };

        params
    }

    // check if tile collides with any other tile of the map
    fn check_collision(&self, t1: &Tile, _map: &Vec<Tile>, coordinates: &Vec2) -> Option<usize> {
        for (index, tile) in self.map.iter().enumerate() {
            if tile.id != t1.id {
                if check_collision_perfect(tile, &coordinates) {
                    return Some(index);
                }
            }
        }
        None
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Option<usize> {
        for (index, tile) in self.map.iter().enumerate() {
            if (tile.position.x / TILE_WIDTH) as usize == x
                && (tile.position.y / TILE_HEIGHT) as usize == y
            {
                return Some(index);
            }
        }
        None
    }

    pub async fn set_level(&mut self, info: LevelInfo) {
        self.map = info.tiles;
        self.blanks = info.blanks;
        self.dimensions = (info.width, info.height);
        self.player.position = (info.width / 2 - 1, info.height / 2);
        self.offset_x = info.offset_x;
        self.offset_y = info.offset_y;
        self.time = info.time;
        self.level = info.level;

    }
    pub async fn new() -> Self {
        let texture_map = load_texture("img/tiles.png").await.unwrap();
        //   set_texture_filter(texture_map, FilterMode::Nearest);
        texture_map.set_filter(FilterMode::Nearest);
        let font = load_ttf_font("Nintendo-NES-Font.ttf").await.unwrap();

        let mut tile_info = HashMap::new();
        tile_info.insert('G', 0u32);
        tile_info.insert('X', 16u32);
        tile_info.insert('E', 32u32);
        tile_info.insert('B', 48u32);
        tile_info.insert('P', 64u32);
        tile_info.insert('C', 80u32);
        tile_info.insert('D', 96u32);
        tile_info.insert('?', 112u32);
        tile_info.insert('~', 112u32);
        tile_info.insert('|', 112u32);
        tile_info.insert('-', 128u32);

        let score_texture = load_texture("img/scoreboard.png").await.unwrap();
        score_texture.set_filter(FilterMode::Nearest);

        let brick_texture = load_texture("img/brick_decoration.png").await.unwrap();
        brick_texture.set_filter(FilterMode::Nearest);

        PlayingState {
            map: vec![],
            blanks: vec![],
            texture_map,
            paused: false,
            exit_intent: false,
            font,
            tile_info,
            player: Player { position: (0, 0) },
            dimensions: (0, 0),
            dragging: false,
            score: 0,
            time_elpsed: 0,
            fading_out: false,
            scoreboard_texture: score_texture,
            brick_decoration: brick_texture,
            offset_x: 0.0,
            offset_y: 0.0,
            level: 0,
            time: 0,
            dragging_step: 0
        }
    }

    pub async fn move_player(&mut self, direction: Direction, mixer: &mut Mixer) {
        let mut new_x: usize = self.player.position.0;
        let mut new_y: usize = self.player.position.1;

        // mixer.play_sound(sound::Sound::MOVE);
        // let mut tile_state = TileState::NONE;
        match direction {
            Direction::Left => {
                if new_x > 0 {
                    new_x -= 1;
                }
            }
            Direction::Right => {
                new_x = usize::min(self.dimensions.0 - 1, new_x + 1);
            }
            Direction::Up => {
                if new_y > 0 {
                     new_y -= 1;
                }
            }
            Direction::Down => new_y = usize::min(self.dimensions.1 - 1, new_y + 1),
            Direction::None => {}
        }

        // let index = self.get_tile_at(new_x, new_y).unwrap();
        // let tile = self.map.get_mut(index).unwrap();

        if self.dragging {
            mixer.play_sound(sound::Sounds::MOVE).await;
            if let Some(index) = self.get_tile_at(self.player.position.0, self.player.position.1) {
                let index2 = self.get_tile_at(new_x, new_y);
                let tile_underneath = self.map.get_mut(index).unwrap();
                if tile_underneath.is_playable()
                    && tile_underneath.looping == false
                    && index2.is_none()
                {
                    tile_underneath.dragging_direction = Some(direction);
                }
            }
        }
        self.player.position.0 = new_x;
        self.player.position.1 = new_y;
    }

    fn new_handle_dragging(&self, tile: &Tile) -> Option<TileChange> {
        if let Some(direction) = &tile.dragging_direction {
            match direction {
                Direction::Left | Direction::Right => {
                    let coordinates = Vec2::new(
                        (self.player.position.0) as f32 * TILE_HEIGHT,
                        (self.player.position.1) as f32 * TILE_HEIGHT,
                    );
                    let collision = self.check_collision(&tile, &self.map, &coordinates);
                    if collision.is_none() {
                        println!("Foo");
                        return Some(TileChange::Jump(Vec2::new(
                            (self.player.position.0) as f32 * TILE_HEIGHT,
                            (self.player.position.1) as f32 * TILE_HEIGHT,
                        )));
                    } else {
                        return Some(TileChange::Stop);
                    }
                }
                Direction::None => return None,
                Direction::Up => return None,
                Direction::Down => return None,
            }
        }
        None
    }

    fn can_push(&self, tile: &Tile, velocity: &Vec2) -> bool {
        // A tile can be pushed/moved to a certan location
        //  if it doesn't collide with anything on that location
        // or, if it collides, that tile can be pushed as well.
        if !tile.is_playable() {
            return false;
        }
        let new_position = tile.position + *velocity;
        if let Some(index) = self.check_collision(&tile, &self.map, &new_position) {
            let collider = self.map.get(index).unwrap();
            return self.can_push(&collider, velocity);
        }
        true
    }
    fn new_handle_collision(&self, tile: &Tile, map: &Vec<Tile>) -> Option<TileChange> {
        // Find out the next theorical coordinates
        let new_position = tile.position + tile.velocity;
        if let Some(index) = self.check_collision(&tile, &map, &new_position) {
            // The tiles is moving and has collided with some other tile
            let collider = self.map.get(index).unwrap();
            if tile.c == '|' || tile.c == '~' {
                if self.can_push(&collider, &tile.velocity) && tile.velocity != Vec2::new(0., SPEED)
                {
                    return None;
                } else {
                    debug!("Cannot push, Tile {} bounces with {}", tile.c, collider.c);
                    return Some(TileChange::Bounce);
                }
            } else if collider.riding {
                return Some(TileChange::StartRiding(collider.velocity.clone()));
            } else if tile.c != '|' && tile.c != '~' {
                debug!("Tile {} stops", tile.c);
                return Some(TileChange::Stop);
            }
        } else {
            // The tile won't collide. If it is not moving, should it fall?
            if tile.velocity == Vec2::ZERO {
                let new_position = tile.position + Vec2::new(0., SPEED);
                if let Some(index) = self.check_collision(&tile, &map, &new_position) {
                    let tile_underneath = self.map.get(index).unwrap();
                    if tile_underneath.c == '|' || tile_underneath.c == '~' {
                        debug!("Tile {} starts riding", tile.c);
                        return Some(TileChange::StartRiding(tile_underneath.velocity.clone()));
                    } else {
                        return Some(TileChange::Stop);
                    }
                } else {
                    // There's nothing underneath, we should fall
                    return Some(TileChange::Fall);
                }
            }
        }
        return Some(TileChange::Move);
    }

    pub fn handle_matches(&self, tile: &Tile) -> Option<TileChange> {
        for t in self.map.iter() {
            if tile.id != t.id
                && tile.c == t.c
                && (((tile.position.x - t.position.x).abs() == TILE_WIDTH)
                    && ((tile.position.y - t.position.y).abs() == 0.0)
                    || ((tile.position.y - t.position.y).abs() == TILE_WIDTH)
                        && ((tile.position.x - t.position.x).abs() == 0.0))
            {
                return Some(TileChange::FadeOut);
            }
        }

        None
    }
    /// Given a map, return all tiles that should change.
    /// That is, which cell (x,y) changes, and the Tile that should be placed there
    pub fn next_map(&self, map: &Vec<Tile>) -> Vec<(usize, TileChange)> {
        let mut changes: Vec<(usize, TileChange)> = vec![];

        for (index, tile) in map.iter().enumerate() {
            if tile.is_playable() {
                if tile.fade_step > 0 {
                    changes.push((index, TileChange::FadeOut));
                } else if self.fading_out == false {
                    if let Some(tc) = self.new_handle_collision(&tile, &map) {
                        changes.push((index, tc));
                    }
                    if let Some(tc) = self.new_handle_dragging(&tile) {
                        changes.push((index, tc));
                    }
                    if let Some(tc) = self.handle_matches(&tile) {
                        changes.push((index, tc));
                    }
                }
            }
        }

        changes
    }
}
