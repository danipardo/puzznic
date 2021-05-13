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

/// AABB collision detection, returns true if collision found
fn check_collision_perfect(
    x1: usize,
    y1: usize,
    t1: &Tile,
    x2: usize,
    y2: usize,
    t2: &Tile,
) -> bool {
    let x1 = x1 as f32 * TILE_WIDTH + t1.position.x as f32;
    let x2 = x2 as f32 * TILE_WIDTH + t2.position.x as f32;

    let y1 = y1 as f32 * TILE_HEIGHT + t1.position.y as f32;
    let y2 = y2 as f32 * TILE_HEIGHT + t2.position.y as f32;

    if (x1 - x2).abs() <= TILE_WIDTH && (y1 - y2).abs() <= TILE_HEIGHT {
//        debug!("Found collision: ({},{}) x ({},{})", x1, y1, x2, y2);
        return true;
    }

    false
}

impl GameState {
    pub fn get_tile_texture_params(&self, c: char) -> DrawTextureParams {
        let offset = *self
            .tile_info
            .get(&c)
            .expect(format!("cannot find tile {}", c).as_str()) as f32;
        // let ratio = screen_width() / screen_height();
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(TILE_WIDTH * 3 as f32, TILE_HEIGHT * 3 as f32)),
            source: Some(Rect::new(offset, 0., TILE_WIDTH as f32, TILE_HEIGHT as f32)),
            rotation: 0.,
            pivot: None,
        };

        params
    }

    pub fn get_tile_at_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        self.map
            .get_mut(y * self.dimensions.0 + x)
            .expect("Tile not found")
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> &Tile {
        self.map.get(y * self.dimensions.0 + x).expect(
            format!(
                "Tile not found at {},{} ({})",
                x,
                y,
                y * self.dimensions.0 + x
            )
            .as_str(),
        )
    }

    pub async fn set_level(&mut self, map: Vec<Tile>, width: usize, height: usize) {
        self.map = map;
        self.dimensions = (width, height);
        self.player.position = (width / 2 - 1, height / 2)
    }
    pub async fn new() -> Self {
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
        tile_info.insert('~', 112u32);
        tile_info.insert('|', 112u32);
        tile_info.insert('-', 128u32);

        GameState {
            map: vec![],
            texture_map,
            tile_info,
            player: Player { position: (0, 0) },
            dimensions: (0, 0),
            dragging: false,
            score: 0,
            time_elpsed: 0,
        }
    }

    pub fn move_player(&mut self, direction: Direction, mixer: &mut Mixer) {
        let x: usize = self.player.position.0;
        let y: usize = self.player.position.1;

        let mut new_x: usize = self.player.position.0;
        let mut new_y: usize = self.player.position.1;

        // mixer.play_sound(sound::Sound::MOVE);
        // let mut tile_state = TileState::NONE;
        match direction {
            Direction::Left => {
                if new_x > 0 {
                    new_x = new_x - 1;
                }
            }
            Direction::Right => {
                new_x = usize::min(self.dimensions.0 - 1, new_x + 1);
            }
            Direction::Up => {
                if new_y > 0 {
                    new_y = new_y - 1;
                }
            }
            Direction::Down => new_y = usize::min(self.dimensions.1 - 1, new_y + 1),
            Direction::None => {}
        }

        let tile_underneath = self.get_tile_at(new_x, new_y).c;
        let ux = self.get_tile_at(new_x, new_y).position.x;
        let uy = self.get_tile_at(new_x, new_y).position.y;
        let fade = self.get_tile_at(new_x, new_y).fade_step;

        if self.dragging {
            let tile = self.map.get_mut(y * self.dimensions.0 + x).unwrap();
            if tile.c != ' ' {
                tile.dragging_direction = Some(direction);
            }
        }
        self.player.position.0 = new_x;
        self.player.position.1 = new_y;
        debug!(
            "Player moved to {:?}: {}, (x: {}, y: {}) Fade: {}",
            self.player.position, tile_underneath, ux, uy, fade
        );
    }

    pub fn check_collision(
        &self,
        x: usize,
        y: usize,
        tile: &Tile,
        direction: &Direction,
    ) -> Option<&Tile> {
        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        match direction {
            Direction::Up => {
                if y > 0 {
                    let t = self.get_tile_at(x, y - 1);
                    if t.c != ' ' && check_collision_perfect(x, y, &tile, x, y - 1, &t) {
                        return Some(&t);
                    }
                    if tile.position.x > 0. && x < map_width {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x + 1, y - 1);
                        if t.c != ' ' && check_collision_perfect(x, y, &tile, x + 1, y - 1, &t) {
                            return Some(&t);
                        }
                    }
                }
            }
            Direction::Down => {
                if y < map_height {
                    let t = self.get_tile_at(x, y + 1);
                    if t.c != ' ' && check_collision_perfect(x, y, &tile, x, y + 1, &t) {
                        return Some(&t);
                    }
                    if tile.position.x > 0. && x < map_width {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x + 1, y + 1);
                        if t.c != ' ' && check_collision_perfect(x, y, &tile, x + 1, y + 1, &t) {
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
                    if t.c != ' ' && check_collision_perfect(x, y, &tile, x - 1, y, &t) {
                        return Some(&t);
                    }
                    if tile.position.y > 0. && y < map_height {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x - 1, y + 1);
                        if t.c != ' ' && check_collision_perfect(x, y, &tile, x - 1, y + 1, &t) {
                            return Some(&t);
                        }
                    }
                }
            }
            Direction::Right => {
                if x < map_width {
                    let t = self.get_tile_at(x + 1, y);
                    if t.c != ' ' && check_collision_perfect(x, y, &tile, x + 1, y, &t) {
                        return Some(&t);
                    }
                    if tile.position.y > 0. && y < map_height {
                        // if its also in the next cell, check the right upper cell
                        let t = self.get_tile_at(x + 1, y + 1);
                        if t.c != ' ' && check_collision_perfect(x, y, &tile, x + 1, y + 1, &t) {
                            return Some(&t);
                        }
                    }
                }
            }
            Direction::None => {}
        }

        None
    }

    fn handle_matchings(
        &self,
        x: usize,
        y: usize,
        tile: &Tile,
        map: &Vec<Tile>,
    ) -> Vec<(usize, usize, TileChange)> {
        let mut changes: Vec<(usize, usize, TileChange)> = vec![];

        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        if tile.position.x == 0. && tile.position.y == 0. && tile.fade_step > 0 {
            changes.push((x, y, TileChange::FadeOut(1)));
        }
        if tile.position.x == 0. && tile.position.y == 0. && tile.fade_step == 0 {
            // Whatever the tile is doing, we have to check in all 4 directions
            // for matches

            if y < map_height
                && self.get_tile_at(x, y + 1).c == tile.c
                && self.get_tile_at(x, y + 1).position.x == 0.
                && self.get_tile_at(x, y + 1).position.y == 0.
            {
                changes.push((x, y, TileChange::Stop));
                changes.push((x, y, TileChange::FadeOut(1)));
            }
            if y > 0
                && self.get_tile_at(x, y - 1).c == tile.c
                && self.get_tile_at(x, y - 1).position.x == 0.
                && self.get_tile_at(x, y - 1).position.y == 0.
            {
                changes.push((x, y, TileChange::Stop));
                changes.push((x, y, TileChange::FadeOut(1)));
            }
            if x < map_width
                && self.get_tile_at(x + 1, y).c == tile.c
                && self.get_tile_at(x + 1, y).position.x == 0.
                && self.get_tile_at(x + 1, y).position.y == 0.
            {
                changes.push((x, y, TileChange::Stop));
                changes.push((x, y, TileChange::FadeOut(1)));
            }
            if x > 0
                && self.get_tile_at(x - 1, y).c == tile.c
                && self.get_tile_at(x - 1, y).position.x == 0.
                && self.get_tile_at(x - 1, y).position.y == 0.
            {
                changes.push((x, y, TileChange::Stop));
                changes.push((x, y, TileChange::FadeOut(1)));
            }
        }

        changes
    }

    fn handle_falling(
        &self,
        x: usize,
        y: usize,
        tile: &Tile,
        map: &Vec<Tile>,
    ) -> Vec<(usize, usize, TileChange)> {
        let mut changes: Vec<(usize, usize, TileChange)> = vec![];

        let map_height = self.dimensions.1;

        if tile.looping == false && tile.riding == false && y < map_height && tile.position.x == 0.
        {
            // If the tile can move, and there is nothing underneath,
            //  it should fall
            let tile_underneath = self.get_tile_at(x, y + 1);
            if tile_underneath.c == ' ' {
                changes.push((x, y, TileChange::VelocityUpdate(Vec2::new(0., SPEED))));
            } else {
                if !check_collision_perfect(x, y, &tile, x, y + 1, &tile_underneath) {
                    // There's something underneath, but we don't collide yet,
                    //  so I can fall freely until colliding
                    changes.push((x, y, TileChange::VelocityUpdate(Vec2::new(0., SPEED))));
                }
            }
            // falling_flag = true;
        }

        changes
    }
    fn handle_dragging(
        &self,
        x: usize,
        y: usize,
        tile: &Tile,
        map: &Vec<Tile>,
    ) -> Vec<(usize, usize, TileChange)> {
        let mut changes: Vec<(usize, usize, TileChange)> = vec![];

        if let Some(direction) = &tile.dragging_direction {
            match direction {
                Direction::Left => {
                    let tile_on_left = self.get_tile_at(x - 1, y);
                    if tile_on_left.c == ' ' {
                        changes.push((x - 1, y, TileChange::Copy(Tile::from(&tile))));
                        changes.push((x, y, TileChange::Copy(Tile::blank())))
                    } else {
                        // force drag stop
                        changes.push((x, y, TileChange::Copy(Tile::from(&tile))));
                    }
                }
                Direction::Right => {
                    let tile_on_right = self.get_tile_at(x + 1, y);
                    if tile_on_right.c == ' ' {
                        changes.push((x + 1, y, TileChange::Copy(Tile::from(&tile))));
                        changes.push((x, y, TileChange::Copy(Tile::blank())));
                    } else {
                        // force drag stop
                        changes.push((x, y, TileChange::Copy(Tile::from(&tile))));
                    }
                }
                Direction::Up => {}
                Direction::Down => {}
                Direction::None => {}
            }
        }

        changes
    }

    // Can the tile Tile, which is currently located at (x,y)
    // move in the direction Direction?
    fn can_move(
        &self,
        tile: &Tile,
        x: usize,
        y: usize,
        direction: &Direction,
        map: &Vec<Tile>,
    ) -> bool {
        let mut other_tile = &Tile::new(' ');
        let mut new_x = x;
        let mut new_y = y;
        match direction {
            Direction::None => {}
            Direction::Left => {
                new_x = x - 1;
                other_tile = self.get_tile_at(x - 1, y);
            }
            Direction::Right => {
                new_x = x + 1;
                other_tile = self.get_tile_at(x + 1, y);
            }
            Direction::Up => {
                new_y = y - 1;
                other_tile = self.get_tile_at(x, y - 1);
            }
            Direction::Down => {
                new_y = y + 1;
                other_tile = self.get_tile_at(x, y + 1);
            }
        }

        if other_tile.c == ' ' {
            return true;
        }
        if other_tile.is_static() {
            return false;
        }
        if other_tile.is_playable() {
            return self.can_move(other_tile, new_x, new_y, direction, map);
        }

        true
    }

    fn handle_movement(
        &self,
        x: usize,
        y: usize,
        tile: &Tile,
        map: &Vec<Tile>,
    ) -> Vec<(usize, usize, TileChange)> {
        let mut changes: Vec<(usize, usize, TileChange)> = vec![];

        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        let mut collision_target = None;

        let mut direction = Direction::None;
        // moving up?
        if tile.velocity == Vec2::new(0., -SPEED) && y > 0 {
            direction = Direction::Up;
            collision_target = self.check_collision(x, y, &tile, &direction);
            if collision_target.is_none() {
                if tile.position.y == 0. {
                    // Jump the sprite from one tile the the one on the top
                    let mut new_tile = Tile::from(&tile);
                    new_tile.position.y = TILE_HEIGHT * 3.;
                    changes.push((x, y, TileChange::Copy(Tile::blank())));
                    changes.push((x, y - 1, TileChange::Copy(new_tile)));
                } else {
                    changes.push((x, y, TileChange::Move));
                }
            } else {
                println!("Moving up and collided!");
            }
        }
        // moving down?
        if tile.velocity == Vec2::new(0., SPEED) && y < map_height {
            direction = Direction::Down;
            collision_target = self.check_collision(x, y, &tile, &direction);
            if collision_target.is_none() {
                if tile.position.y >= TILE_WIDTH * 3. {
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
            direction = Direction::Left;
            collision_target = self.check_collision(x, y, &tile, &direction);

            if collision_target.is_none() {
                if tile.position.x == 0. {
                    let mut new_tile = Tile::from(&tile);
                    new_tile.riding = false;
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
            direction = Direction::Right;
            collision_target = self.check_collision(x, y, &tile, &direction);
            if collision_target.is_none() {
                if tile.position.x > TILE_WIDTH * 3. {
                    let mut new_tile = Tile::from(&tile);
                    new_tile.position.x = 0.;
                    new_tile.riding = false;
                    changes.push((x + 1, y, TileChange::Copy(new_tile)));
                    changes.push((x, y, TileChange::Copy(Tile::blank())));
                } else {
                    changes.push((x, y, TileChange::Move));
                }
            }
        }

        if collision_target.is_some() {
            let collider = collision_target.unwrap();
            println!(
                "I am {}: Collision at ({},{}) with {}, riding: {}",
                tile.c, x, y, collider.c, collider.riding
            );
            // If the tile is riding and moving up, the tile will be able to move up
            // as long as the tile on top can also move up
            if tile.looping {
                if !self.can_move(tile, x, y, &direction, &map) {
                    changes.push((x, y, TileChange::Bounce));
                    //   debug!("Woops, I am a looping platform and will now bounce");
                }
            } else if tile.riding {
                // if I cannot move upwards, I should start falling
                if !self.can_move(tile, x, y, &direction, &map) {
                    changes.push((x, y, TileChange::RidingFlag(false)));
                    //                    changes.push((x, y, TileChange::VelocityUpdate(Vec2::new(0., SPEED))));
                    changes.push((x, y, TileChange::Stop));
                }
            } else if collider.riding {
                changes.push((x, y, TileChange::RidingFlag(true)));
                changes.push((x, y, TileChange::VelocityUpdate(collider.velocity)));
            } else if tile.looping && collider.is_static() {
                // If I am a moving platform, bounce
                changes.push((x, y, TileChange::Bounce));
            } else {
                changes.push((x, y, TileChange::Stop));
            }
        }

        changes
    }

    /// Given a map, return all tiles that should change.
    /// That is, which cell (x,y) changes, and the Tile that should be placed there
    pub fn next_map(&self, map: &Vec<Tile>) -> Vec<(usize, usize, TileChange)> {
        let mut changes: Vec<(usize, usize, TileChange)> = vec![];

        let map_width = self.dimensions.0;
        let map_height = self.dimensions.1;

        for y in 0..map_height {
            for x in 0..map_width {
                // let mut _falling_flag = false;
                let tile = map.get(y * map_width + x).unwrap();

                if tile.is_static() {
                    continue;
                }

                changes.append(&mut self.handle_dragging(x, y, tile, map));
                changes.append(&mut self.handle_falling(x, y, tile, map));
                changes.append(&mut self.handle_matchings(x, y, tile, map));
                changes.append(&mut self.handle_movement(x, y, tile, map));
            }
        }
        changes
    }
}
