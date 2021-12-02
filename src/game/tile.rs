use macroquad::prelude::Vec2;

use super::playing_state::Direction;

#[derive(Debug)]
pub struct Tile {
    pub id: u32,
    pub c: char,
    pub fade_step: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub looping: bool,
    pub riding: bool,
    pub dragging_direction: Option<Direction>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Tile {
    pub fn new(id: u32, c: char, x: f32, y: f32) -> Tile {
        let mut t = Tile {
            id: id,
            c: c,
            position: Vec2::new(x, y),
            velocity: Vec2::zero(),
            looping: false,
            dragging_direction: None,
            riding: false,
            fade_step: 0,
        };

        t.position = Vec2::new(x, y);
        t.c = c;
        if c == '|' || c == '~' {
            t.riding = true;
        }
        return t;
    }

    pub fn is_playable(&self) -> bool {
        return self.c != ' ' && self.c != '-';
    }
}

pub enum TileChange {
    Move,
    Bounce,
    Stop,
    FadeOut,
    // Copy(Tile),
    Jump(Vec2),
    VelocityUpdate(Vec2),
    StartRiding(Vec2),
    Fall,
    RidingFlag(bool),
}
