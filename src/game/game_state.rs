use std::{borrow::BorrowMut, collections::HashMap, hint::unreachable_unchecked, rc::Rc};

use super::*;

pub struct GameState {
    pub map: Vec<Tile>,
    pub texture_map: Texture2D, // single image that contains all the tiles
    pub tile_info: HashMap<char, u32>, // image offset of each tile in the main image
    pub dimensions: (usize, usize), // map dimensions
    pub player: Player,
    pub score: u32,
    pub time_elpsed: u32,
    pub dragging: bool,
}

impl GameState {
    pub fn get_tile_texture_params(&self, c: char) -> DrawTextureParams {
        let offset = *self.tile_info.get(&c).unwrap() as f32;
        // let ratio = screen_width() / screen_height();
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(TILE_WIDTH * 3 as f32, TILE_HEIGHT * 3 as f32)),
            source: Some(Rect::new(offset, 0., TILE_WIDTH as f32, TILE_HEIGHT as f32)),
            rotation: 0.,
            pivot: None,
        };

        params
    }

    // Returns the window coordinates of the coresponding tile position (x,y)
    // pub fn tile_to_coords(&self, x: usize, y: usize) -> (f32, f32) {
    //     let x = screen_width() / 2. - self.dimensions.0 as f32 / 2. * TILE_WIDTH
    //         + x as f32 * TILE_WIDTH;
    //     let y = screen_height() / 2. - self.dimensions.1 as f32 / 2. * TILE_HEIGHT
    //         + y as f32 * TILE_HEIGHT;

    //     (x, y)
    // }

    pub fn get_tile_at(&self, x: usize, y: usize) -> &Tile {
        self.map
            .get(y * self.dimensions.0 + x)
            .expect("Tile not found")
    }

    pub async fn new(level: &String) -> Self {
        let mut map: Vec<Tile> = vec![];

        let lines = level.split("\n");

        let rows: usize = lines.count();
        let columns = &level
            .split("\n")
            .into_iter()
            .map(|e| e.chars().count())
            .max()
            .unwrap();

        let lines = level.split("\n");
        // let mut start_x = 0.0;
        // let mut start_y = 0.0;

        for line in lines {
            for c in line.chars() {
                let mut t: Tile = Tile {
                    c,
                    position: Vec2::new(0., 0.),
                    // slide_step: 0,
                    velocity: Vec2::new(0., 0.),
                    position_changed: false,
                    looping: false,
                };

                if c == 'w' {
                    t.velocity = Vec2::new(0., -SPEED);
                    t.looping = true
                }


                map.push(t);
                // start_x = start_x + TILE_WIDTH as f32;
            }
            // start_y = start_y + TILE_HEIGHT as f32;
            // start_x = 0.0;
        }

        let texture_map = load_texture("img/tiles.png").await;
        set_texture_filter(texture_map, FilterMode::Nearest);

        let mut tile_info = HashMap::new();
        tile_info.insert('G', 0u32);
        tile_info.insert('X', 16u32);
        tile_info.insert('E', 32u32);
        tile_info.insert('B', 48u32);
        tile_info.insert('P', 64u32);
        tile_info.insert('C', 80u32);
        tile_info.insert('D', 96u32);
        tile_info.insert('?', 112u32);
        tile_info.insert('-', 128u32);

        GameState {
            map,
            texture_map,
            tile_info,
            player: Player {
                position: (columns / 2, rows / 2),
            },
            dimensions: (*columns, rows),
            dragging: false,
            score: 0,
            time_elpsed: 0,
        }
    }

    pub fn move_player(&mut self, direction: Direction) {
        let x: usize = self.player.position.0;
        let y: usize = self.player.position.1;

        let mut new_x: usize = self.player.position.0;
        let mut new_y: usize = self.player.position.1;

        // let mut tile_state = TileState::NONE;
        let mut velocity = Vec2::new(0., 0.);
        match direction {
            Direction::Left => {
                if new_x > 0 {
                    new_x = new_x - 1;
                    velocity = Vec2::new(-SPEED, 0.);
                }
            }
            Direction::Right => {
                new_x = usize::min(self.dimensions.0, new_x + 1);
                velocity = Vec2::new(SPEED, 0.);
            }
            Direction::Up => {
                if new_y > 0 {
                    new_y = new_y - 1;
                }
            }
            Direction::None => {}
            Direction::Down => new_y = usize::min(self.dimensions.1, new_y + 1),
        }

        let tile_underneath = self.get_tile_at(new_x, new_y).c;
        if tile_underneath == '-' {
            return;
        }
        if self.dragging {
             let tile = self.map.get_mut(y * self.dimensions.0 + x).unwrap();
            if tile.c != 'x' {
                tile.velocity = velocity;
            }
        }
        self.player.position.0 = new_x;
        self.player.position.1 = new_y;
        debug!("Player moved to {:?}", self.player.position);
    }

    

    pub fn check_collision(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let _tile1 = self.map.get(y1 * self.dimensions.0 + x1).unwrap();
        let tile2 = self.map.get(y2 * self.dimensions.0 + x2);

        if tile2.is_none() {
            return false;
        }
        let _tile2 = tile2.unwrap();

        false
    }

    // Returns a tile index + a list of tile indexes that collide with it
    pub fn get_collisions(&self) -> Vec<(usize, Vec<usize>)> {
        vec![]
    }
    /// Given a map, return all tiles that should change.
    /// That is, which cell (x,y) changes, and the Tile that should be placed there
    pub fn next_map(&self, map: &Vec<Tile>) -> Vec<(usize, usize, Tile)> {
        let changes: Vec<(usize, usize, Tile)> = vec![];

        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        for y in 0..map_height {
            for x in 0..map_width {
                let tile = map.get(y * map_width + x).unwrap();
                // Changes should only trigger when the tile has finished the whole move transition

                if tile.velocity == Vec2::new(0.0, 0.0) {
                    continue;
                }

                // moving up?
                if tile.velocity == Vec2::new(0., -SPEED) && y > 0 {
                    let _collides = self.check_collision(x, y, x, y - 1);
                    // new_y -= 1;
                }
                // moving down?
                if tile.velocity == Vec2::new(0., SPEED) && y < map_height {
                    let _collides = self.check_collision(x, y, x, y + 1);
                }

                // moving left
                if tile.velocity == Vec2::new(-SPEED, 0.) && x > 0 {
                    let _collides = self.check_collision(x - 1, y, x, y);
                }
                if tile.velocity == Vec2::new(SPEED, 0.) && x < map_width {
                    let _collides = self.check_collision(x + 1, y, x, y);
                }
            }
        }

        changes
    }
}
