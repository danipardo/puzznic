use macroquad::prelude::Vec2;

use super::Direction;

#[derive(Debug)]
pub struct Tile {
    pub c: char,
    pub fade_step: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub position_changed: bool,
    pub looping: bool,
    pub riding: bool,
    pub dragging_direction: Option<Direction>,
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


pub enum TileChange {
    Move,
    Bounce,
    Stop,
    FadeOut(usize),
    Copy(Tile),
    VelocityUpdate(Vec2),
    RidingFlag(bool),
}
