use std::collections::HashMap;

use macroquad::*;

const TILE_WIDTH: f32 = 48f32;
const TILE_HEIGHT: f32 = 48f32;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub c: char,
}
pub struct TileMap {
    pub map: Vec<Vec<Tile>>,
    pub texture_map: Texture2D,
    pub tile_info: HashMap<char, u32>,
    pub dimensions: (usize, usize),
    pub player: Player,
}

pub struct Player {
    pub position: (usize, usize),
}

impl TileMap {
    fn get_tile(&self, c: char) -> DrawTextureParams {
        let offset = *self.tile_info.get(&c).unwrap() as f32;
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32)),
            source: Some(macroquad::Rect::new(
                offset,
                0.,
                TILE_WIDTH as f32,
                TILE_HEIGHT as f32,
            )),
            rotation: 0.,
        };

        params
    }

    // Returns the window coordinates of the coresponding tile position (x,y)
    pub fn tile_to_coords(&self, x: usize, y: usize) -> (f32, f32) {
        let x = screen_width() / 2. - self.dimensions.0 as f32 / 2. * TILE_WIDTH + x as f32 * TILE_WIDTH;
        let y = screen_height() / 2. - self.dimensions.1 as f32 / 2. * TILE_HEIGHT + y as f32 * TILE_HEIGHT;

        (x, y)
    }

    pub fn draw(&self) {
        let dimensions = self.dimensions;

        let mut start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
        let mut start_y = screen_height() / 2. - dimensions.1 as f32 * TILE_HEIGHT / 2. as f32;

        for y in 0..dimensions.0 {
            for x in 0..8 {
                //debug!(" x: {}, y: {}", start_x, start_y);
                let tile = self.map.get(x).unwrap().get(y).expect("Tile not found");
                if tile.c != 'x' {
                    draw_texture_ex(
                        self.texture_map,
                        start_x,
                        start_y,
                        WHITE,
                        self.get_tile(tile.c),
                    );
                }
                start_x = start_x + TILE_HEIGHT as f32;
            }
            //            start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
            start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
            start_y = start_y + TILE_WIDTH;
        }

        // Draw player rectangle
        debug!("Player on {:?}", self.player.position);
        let (x, y) = self.tile_to_coords(self.player.position.0, self.player.position.1);

        debug!("Player on {},{}", x, y);
        draw_rectangle_lines(x, y, TILE_WIDTH, TILE_HEIGHT, 8., RED);
    }
    pub async fn new(level: &String) -> Self {
        let mut map: Vec<Vec<Tile>> = vec![];

        let lines = level.split("\n");

        let mut rows: usize = 0;
        let mut columns: usize = 0;
        for line in lines {
            let mut s1 = vec![];
            for c in line.chars() {
                let t: Tile = Tile { c };
                s1.push(t);
            }
            rows = rows + 1;
            let length = s1.len();
            columns = std::cmp::max(columns, length);
            map.push(s1);
        }

        let texture_map = macroquad::load_texture("img/tiles.png").await;

        let mut tile_info = HashMap::new();
        tile_info.insert('G', 0u32);
        tile_info.insert('X', 48u32);
        tile_info.insert('E', 96u32);
        tile_info.insert('B', 144u32);
        tile_info.insert('P', 192u32);
        tile_info.insert('C', 240u32);
        tile_info.insert('D', 288u32);
        tile_info.insert('?', 336u32);
        tile_info.insert('-', 384u32);

        TileMap {
            map,
            texture_map,
            tile_info,
            player: Player {
                position: (columns / 2, rows / 2),
            },
            dimensions: (columns, rows),
        }
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
