use std::collections::HashMap;

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
            Direction::Down => new_y = usize::min(self.dimensions.1, new_y + 1),
        }

        let tile_underneath = self.get_tile_at(new_x, new_y).c;
        let ux = self.get_tile_at(new_x, new_y).position.x;
        let uy = self.get_tile_at(new_x, new_y).position.y;
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
        debug!(
            "Player moved to {:?}: {}, (x: {}, y: {})",
            self.player.position, tile_underneath, ux, uy
        );
    }

    pub fn check_collision(
        &self,
        x: usize,
        y: usize,
        tile: &Tile,
        direction: Direction,
    ) -> Option<&Tile> {
        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        match direction {
            Direction::Up => {
                if y > 0 {
                    let t = self.get_tile_at(x, y - 1);
                    if t.c != 'x' {
                        return Some(&t);
                    }
                    if tile.position.x > 0. && x < map_width {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x + 1, y - 1);
                        if t.c != 'x' {
                            return Some(&t);
                        }
                    }
                }
            }
            Direction::Down => {
                if y < map_height {
                    let t = self.get_tile_at(x, y + 1);
                    if t.c != 'x' {
                        return Some(&t);
                    }
                    if tile.position.x > 0. && x < map_width {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x + 1, y + 1);
                        if t.c != 'x' {
                            return Some(&t);
                        }
                    }
                }
            }
            Direction::Left => {
                println!(
                    "Checking left at {},{} position ({},{})",
                    x, y, tile.position.x, tile.position.y
                );
                if x > 0 && tile.position.x == 0. {
                    let t = self.get_tile_at(x - 1, y);
                    if t.c != 'x' {
                        return Some(&t);
                    }
                    if tile.position.y > 0. && y < map_height {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x - 1, y + 1);
                        if t.c != 'x' {
                            return Some(&t);
                        }
                    }
                }
            }
            Direction::Right => {
                if x < map_width {
                    let t = self.get_tile_at(x + 1, y);
                    if t.c != 'x' {
                        return Some(&t);
                    }
                    if tile.position.y > 0. && y < map_height {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x + 1, y + 1);
                        if t.c != 'x' {
                            return Some(&t);
                        }
                    }
                }
            }
        }

        None
    }

    /// Given a map, return all tiles that should change.
    /// That is, which cell (x,y) changes, and the Tile that should be placed there
    pub fn next_map(&self, map: &Vec<Tile>) -> Vec<(usize, usize, TileChange)> {
        let mut changes: Vec<(usize, usize, TileChange)> = vec![];

        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        for y in 0..map_height {
            for x in 0..map_width {

                let mut falling_flag = false;
                let tile = map.get(y * map_width + x).unwrap();
                let mut collision_target = None;

                if tile.is_static() {
                    continue;
                }

                if y < map_height && tile.position.x == 0. && self.get_tile_at(x, y + 1).c == 'x' {
                    // If the tile can move, and there is nothing underneath,
                    //  it should fall
                    changes.push((x, y, TileChange::VelocityUpdate(Vec2::new(0., SPEED))));
                    falling_flag = true;
                }


                // moving up?
                if tile.velocity == Vec2::new(0., -SPEED) && y > 0 {
                    collision_target = self.check_collision(x, y, &tile, Direction::Up);
                }
                // moving down?
                if tile.velocity == Vec2::new(0., SPEED) && y < map_height {
                    collision_target = self.check_collision(x, y, &tile, Direction::Down);
                    if collision_target.is_none() {
                        if tile.position.y > TILE_WIDTH * 3. {
                            let mut new_tile = Tile::from(&tile);
                            new_tile.position.y = 0.;
                            changes.push((x, y, TileChange::Copy(Tile::blank())));
                            changes.push((x, y + 1, TileChange::Copy(new_tile)));
                        } else {
                            changes.push((x, y, TileChange::Move));
                        }
                    }
                }

                // moving left
                if tile.velocity == Vec2::new(-SPEED, 0.) && x > 0 {
                    collision_target = self.check_collision(x, y, &tile, Direction::Left);

                    if collision_target.is_none() {
                        if tile.position.x == 0. && !falling_flag {
                            let mut new_tile = Tile::from(&tile);
                            new_tile.position.x = TILE_WIDTH * 3. - 1.;
                            changes.push((x - 1, y, TileChange::Copy(new_tile)));
                            changes.push((x, y, TileChange::Copy(Tile::blank())));
                        } else {
                            changes.push((x, y, TileChange::Move));
                        }
                    }
                }

                // moving right
                if tile.velocity == Vec2::new(SPEED, 0.) && x < map_width {
                    collision_target = self.check_collision(x, y, &tile, Direction::Right);
                    if collision_target.is_some() {
                        println!("Collision with {:?}", &collision_target);
                    }

                    if collision_target.is_none() {
                        if tile.position.x > TILE_WIDTH * 3. {
                            let mut new_tile = Tile::from(&tile);
                            new_tile.position.x = 0.;
                            changes.push((x + 1, y, TileChange::Copy(new_tile)));
                            changes.push((x, y, TileChange::Copy(Tile::blank())));
                        } else {
                            changes.push((x, y, TileChange::Move));
                        }
                    }
                }
                
                if collision_target.is_some() {
                    println!(
                        "Collision at ({},{}) with {}",
                        x,
                        y,
                        collision_target.unwrap().c
                    );
                    if tile.c == collision_target.unwrap().c {
                        // if my tile is the same as the colliding one, fadeout
                        changes.push((x, y, TileChange::FadeOut(1)));
                    } else if tile.c == 'H' {
                        // If I am a moving platform, bounce
                        changes.push((x, y, TileChange::Bounce));
                    } else {
                        changes.push((x, y, TileChange::Stop));
                    }
                }


            }
        }

        changes
    }
}
