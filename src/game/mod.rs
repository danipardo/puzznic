use std::collections::HashMap;

use macroquad::*;

const TILE_WIDTH: f32 = 16f32;
const TILE_HEIGHT: f32 = 16f32;


#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub c: char,
}

pub struct TileMap {
    pub map: Vec<Vec<Tile>>,
    pub texture_map: Texture2D,
    pub tile_info : HashMap<char, u32>
}

fn get_tile(c: char) -> DrawTextureParams {
    let params = DrawTextureParams {
        dest_size: Some(Vec2::new(16., 16.)),
        source: Some(macroquad::Rect::new(0., 0., 16., 16.)),
        rotation: 0.,
    };

    params
}

impl TileMap {
    pub fn dimensions(&self) -> (usize, usize) {
        let mut width: usize = 0;
        let height: usize = self.map.len();
        for x in &self.map {
            width = std::cmp::max(width, x.len());
        }

        (width, height)
    }

    pub fn draw(&self) {
        let dimensions = self.dimensions();

        let mut start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
        let mut start_y = screen_height() / 2. - dimensions.1 as f32 * TILE_HEIGHT / 2. as f32;

        debug!("Dimensions: {} {}", dimensions.0, dimensions.1);

        
        for y in 0..dimensions.0 {
            for x in 0..8 {
                debug!(" x: {}, y: {}", x, y);
                let tile = self.map.get(x).
                unwrap().get(y).expect("Tile not found");
                if tile.c!='x'{
                draw_texture_ex(self.texture_map, start_x, start_y, WHITE, get_tile(tile.c));
                }
                start_x = start_x + TILE_HEIGHT as f32;
            }
            start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
            start_y = start_y + TILE_WIDTH;
        }
    }
    pub async fn new(level: &String) -> Self {
        let mut map: Vec<Vec<Tile>> = vec![];

        let lines = level.split("\n");

        for line in lines {
            let mut s1 = vec![];
            for c in line.chars() {
                let t: Tile = Tile { c };

                s1.push(t);
            }

            map.push(s1);
        }

        let texture_map = macroquad::load_texture("img/tiles.png").await;

        let mut tile_info = HashMap::new();
        tile_info.insert('G', 0u32);
        tile_info.insert('X', 16u32);
        tile_info.insert('E', 32u32);
        tile_info.insert('B', 48u32);
        tile_info.insert('P', 64u32);
        tile_info.insert('C', 16u32);
        tile_info.insert('D', 16u32);
        
        tile_info.insert('-', 16u32);


        TileMap { map, texture_map, tile_info }
    }
}

pub async fn play_level(level: &TileMap) {
    loop {
        clear_background(GRAY);

        level.draw();

        if macroquad::is_key_down(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
