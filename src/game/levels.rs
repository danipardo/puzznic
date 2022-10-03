use super::tile::*;
use crate::game::{
    game_logic::LevelInfo,
    playing_state::{SPEED, TILE_HEIGHT, TILE_WIDTH},
};
use macroquad::prelude::*;
use regex::Regex;

pub fn load_level(n: usize) -> LevelInfo {
    let s = std::fs::read_to_string(format!("levels/{}.txt", n)).unwrap();
    let tokens: Vec<&str> = s.split("\n").collect();

    let time = extract_seconds(&tokens[2]);
    let tokens = &tokens[4..];
    let mut map = vec![];
    let mut blanks = vec![]; // a vec of blank tiles, to draw the background

    let mut rows: Vec<String> = tokens
        .iter()
        .map(|s| s.chars().step_by(2).collect::<String>())
        .filter(|r| !r.is_empty())
        .collect();

//    println!("{:?}", rows);

    let map_height = rows.len();
    let map_width = &rows.iter().map(|c| c.len()).max().unwrap();
    let mut tile_index = 1;
    for (y, line) in rows.iter_mut().enumerate() {
        for _ in 0..map_width - line.len() {
            line.push(' ');
        }

        // get a vec of tuples, where tuple.0 is the index
        println!("line=>{}", line);
        let v: Vec<_> = line.match_indices('-').collect();
        debug_assert!(!v.is_empty());
        let first_brick_idx = v[0].0;
        let last_brick_idx = v.last().unwrap().0;

        println!("First: {}, Last: {}", first_brick_idx, last_brick_idx);
        for (x, c) in line.chars().enumerate() {
            if c != '-' && x > first_brick_idx && x < last_brick_idx {
                blanks.push(Tile::new(
                    tile_index,
                    c,
                    x as f32 * TILE_HEIGHT,
                    y as f32 * TILE_HEIGHT,
                ));
            }
            if c == ' ' {
                continue;
            }

            let mut t = Tile::new(
                tile_index,
                c,
                x as f32 * TILE_HEIGHT,
                y as f32 * TILE_HEIGHT,
            );

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
            if c != ' ' {
                map.push(t);
                tile_index += 1;
            }
        }
    }

    let offset_y = (200. - map_height as f32 * TILE_HEIGHT) / 2.;
    let offset_x = (320. + 100. - *map_width as f32 * TILE_WIDTH) / 2.;
    let info = LevelInfo {
        tiles: map,
        blanks,
        width: *map_width,
        height: map_height,
        offset_x,
        offset_y,
        level: n,
        time: time,
    };

    info
}

fn extract_seconds(str: &str) -> u16 {
    let re = Regex::new(r"Time: (\d)'(\d{2})").unwrap();
    let captures = re.captures(str).unwrap();
    let minutes: u16 = captures.get(1).unwrap().as_str().parse().unwrap();
    let seconds: u16 = captures.get(2).unwrap().as_str().parse().unwrap();

    println!("Found time: {},{}", minutes, seconds);
    return minutes * 60 + seconds;
}
