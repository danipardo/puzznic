
use crate::game::playing_state::{SPEED, TILE_HEIGHT};
use super::tile::*;
use macroquad::prelude::*;

pub fn load_level(n: u32) -> (Vec<Tile>, usize, usize) {
    let s = std::fs::read_to_string(format!("levels/{}.txt", n)).unwrap();
    let tokens: Vec<&str> = s.split("\n").collect();

    let mut map = vec![];
    // let x : String =   "xxx".chars().step_by(2).collect();
    let mut rows: Vec<String> = tokens
        .iter()
        .map(|s| s.chars().step_by(2).collect::<String>())
        .collect();

    println!("{:?}", rows);

    let map_height = rows.len();
    let map_width = &rows.iter().map(|c| c.len()).max().unwrap();
    let mut tile_index = 1;
    for (y, line) in rows.iter_mut().enumerate() {
        for _ in 0..map_width - line.len() {
            line.push(' ');
        }
        for (x, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            let mut t = Tile::new(tile_index, c, x as f32 * TILE_HEIGHT, y as f32 * TILE_HEIGHT);
            if c == '|' {
                t.velocity = Vec2::new(0., -SPEED);
                t.looping = true;
                t.riding = true;
            }
            if c == '~' {
                t.velocity = Vec2::new(SPEED, 0.);
                t.looping = true;
                t.riding = true;
            }

            map.push(t);
            tile_index +=1;
        }
    }

    // println!("Map dimensions : {},{}, tiles: {}", map_width, map_height, map.len());
     (map, *map_width, map_height)
}
