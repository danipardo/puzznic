use macroquad::*;
use std::collections::HashMap;

const TILE_WIDTH: f32 = 48f32;
const TILE_HEIGHT: f32 = 48f32;

#[derive(Debug)]
pub struct Tile {
    pub c: char,
    slide_step: u32,
    position: Vec2,
    velocity: Vec2,
    position_changed: bool,
    direction: Direction,
}

pub enum TileTransform {
    Position(char), // a tile has moved from one cell to the next one
    Velocity(Vec2), // A tile changes its velocity (i.e when its moving sideways and starts falling down)
}

impl Tile {
    pub fn do_move(&mut self) -> bool {
        if self.velocity == Vec2::zero() {
            return false;
        }
        self.position = self.position + self.velocity;

        debug!("{:?}", self.position);
        self.slide_step += 1;
        if self.slide_step == TILE_HEIGHT as u32 {
            self.position_changed = true;
            return true;
        } else {
            self.position_changed = false;
        }
        false
    }
    pub fn stop(&mut self) {
        self.velocity = Vec2::new(0., 0.);
        self.direction = Direction::None;
        self.slide_step = 0;
    }
    pub fn _reverse_direction(&mut self) {
        self.velocity.set_x(self.velocity.x() * -1.);
        self.velocity.set_y(self.velocity.y() * -1.);
    }
}
pub struct TileMap {
    pub map: Vec<Tile>,
    pub texture_map: Texture2D, // single image that contains all the tiles
    pub tile_info: HashMap<char, u32>, // image offset of each tile in the main image
    pub dimensions: (usize, usize), // map dimensions
    pub player: Player,
    pub dragging: bool,
}

pub struct Player {
    pub position: (usize, usize),
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
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
        let x = screen_width() / 2. - self.dimensions.0 as f32 / 2. * TILE_WIDTH
            + x as f32 * TILE_WIDTH;
        let y = screen_height() / 2. - self.dimensions.1 as f32 / 2. * TILE_HEIGHT
            + y as f32 * TILE_HEIGHT;

        (x, y)
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> &Tile {
        self.map
            .get(y * self.dimensions.0 + x)
            .expect("Tile not found")
    }

    pub fn draw(&self) -> bool {
        let dimensions = self.dimensions;
        let mut start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
        let mut start_y = screen_height() / 2. - dimensions.1 as f32 * TILE_HEIGHT / 2. as f32;

        let mut finished = true;
        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                let tile = self.get_tile_at(x, y);
                if tile.c != 'x' && tile.c != '-' {
                    finished = false;
                }
                if tile.c != 'x' {
                    draw_texture_ex(
                        self.texture_map,
                        start_x + tile.position.x(),
                        start_y + tile.position.y(),
                        WHITE,
                        self.get_tile(tile.c),
                    );
                }
                start_x = start_x + TILE_HEIGHT as f32;
            }

            start_x = screen_width() / 2. - dimensions.0 as f32 * TILE_WIDTH / 2. as f32;
            start_y = start_y + TILE_WIDTH;
        }

        finished
    }

    pub fn draw_player(&self) {
        // Draw player rectangle
        let (x, y) = self.tile_to_coords(self.player.position.0, self.player.position.1);

        draw_rectangle_lines(x, y, TILE_WIDTH, TILE_HEIGHT, 8., RED);
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
        let mut start_x = screen_width() / 2. - *columns as f32 * TILE_WIDTH / 2. as f32;
        let mut start_y = screen_height() / 2. - rows as f32 * TILE_HEIGHT / 2. as f32;

        for line in lines {
            for c in line.chars() {
                let t: Tile = Tile {
                    c,
                    position: Vec2::new(0., 0.),
                    slide_step: 0,
                    velocity: Vec2::new(0., 0.),
                    position_changed: false,
                    direction: Direction::None,
                };
                map.push(t);
                start_x = start_x + TILE_WIDTH as f32;
            }
            start_y = start_y + TILE_HEIGHT as f32;
            start_x = screen_width() / 2. - *columns as f32 * TILE_WIDTH / 2. as f32;
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
            dimensions: (*columns, rows),
            dragging: false,
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
                    velocity = Vec2::new(-1., 0.);
                }
            }
            Direction::Right => {
                new_x = usize::min(self.dimensions.0, new_x + 1);
                velocity = Vec2::new(1., 0.);
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
        debug!("Found: {}", tile_underneath);
        if tile_underneath == '-' {
            return;
        }
        if self.dragging {
            let tile = self.map.get_mut(y * self.dimensions.0 + x).unwrap();
            if tile.c != 'x' {
                tile.velocity = velocity;
                tile.direction = direction;
            }
        }
        debug!("Moving to {},{}, dragging: {}", new_x, new_y, self.dragging);
        self.player.position.0 = new_x;
        self.player.position.1 = new_y;
    }

    /// For each tile, check if it collides with any other tile
    /// Also, if there's nothing beneath, it should fall
    pub fn move_tiles(&mut self) {
        for y in 0..self.dimensions.0 {
            for x in 0..self.dimensions.1 {
                let tile = self.map.get_mut(y * self.dimensions.0 + x).unwrap();
                let _changed_cell = tile.do_move();
            }
        }

        let changes = self.next_map(&self.map);

        for change in &changes {
            let t = self
                .map
                .get_mut(change.1 * self.dimensions.0 + change.0)
                .unwrap();
            match change.2 {
                TileTransform::Position(c) => {
                    t.c = c;
                    t.position_changed = false;
                    t.slide_step = 0;
                    t.direction = Direction::None;
                    t.velocity = Vec2::new(0., 0.);
                    t.position = Vec2::new(0., 0.);
                }
                TileTransform::Velocity(velocity) => {
                    debug!("Changing velocity of tile {} to {:?}", t.c, velocity);
                    t.direction = Direction::Down;
                    t.velocity = velocity;
                    t.slide_step = 0;
                    t.position_changed = false;
                }
            }
        }
        if changes.len() > 0 {
            self.draw_map(&self.map);
        }
    }

    pub fn draw_map(&self, map: &Vec<Tile>) {
        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        for y in 0..map_height {
            for x in 0..map_width {
                let tile = map.get(y * map_width + x).unwrap();
                print!("{}", tile.c)
            }
            println!();
        }
    }

    /// Given a map, return all tiles that should change.
    /// That is, which cell (x,y) changes, and the Tile that should be placed there
    pub fn next_map(&self, map: &Vec<Tile>) -> Vec<(usize, usize, TileTransform)> {
        let mut changes: Vec<(usize, usize, TileTransform)> = vec![];

        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        for y in 0..map_height {
            for x in 0..map_width {
                let tile = map.get(y * map_width + x).unwrap();
                // Changes should only trigger when the tile has finished the whole move transition
                if !tile.position_changed {
                    continue;
                }

                let mut new_x = x;
                let mut new_y = y;
                // Apply the new position on the changeset
                match tile.direction {
                    Direction::Up => {
                        if y > 0 {
                            new_y -= 1;
                        }
                    }
                    Direction::Down => {
                        if y < map_height {
                            new_y += 1;
                        }
                    }
                    Direction::Left => {
                        if x > 0 {
                            new_x -= 1;
                        }
                    }
                    Direction::Right => {
                        if x < map_width {
                            new_x += 1;
                        }
                    }
                    Direction::None => {}
                }
                changes.push((new_x, new_y, TileTransform::Position(tile.c)));
                changes.push((x, y, TileTransform::Position('x')));

                // Check if the tile has to fall
                let underneath = map.get((new_y + 1) * map_width + new_x).unwrap();
                if underneath.c == 'x' {
                    debug!("Tile at {},{} should fall", x, y);
                    changes.push((new_x, new_y, TileTransform::Velocity(Vec2::new(0., 1.))));
                }
            }
        }

        changes
    }
}

pub async fn play_level(level: &mut TileMap) {
    loop {
        clear_background(GRAY);

        let finished = level.draw();
        if finished {
            break;
        }
        level.draw_player();
        if macroquad::is_key_down(KeyCode::Escape) {
            break;
        }

        if macroquad::is_key_pressed(KeyCode::Left) {
            level.move_player(Direction::Left);
        }
        if macroquad::is_key_pressed(KeyCode::Right) {
            level.move_player(Direction::Right);
        }

        if macroquad::is_key_pressed(KeyCode::Up) && !level.dragging {
            level.move_player(Direction::Up);
        }

        if macroquad::is_key_pressed(KeyCode::Down) {
            level.move_player(Direction::Down);
        }

        level.dragging = macroquad::is_key_down(KeyCode::Space);

        level.move_tiles();
        next_frame().await;
    }
}
