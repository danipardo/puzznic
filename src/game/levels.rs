use super::*;

pub fn load_level(n: u32) -> (Vec<Tile>, usize, usize) {
    let s = std::fs::read_to_string(format!("levels/{}.txt", n)).unwrap();
    let tokens: Vec<&str> = s.split("\n").collect();

    let mut map = vec![];
    // let x : String =   "xxx".chars().step_by(2).collect();
    let rows: Vec<String> = tokens
        .iter()
        .map(|s| s.chars().step_by(2).collect::<String>())
        .collect();

    println!("{:?}", rows);

    let map_height = rows.len();
    let map_width = &rows.iter().map(|c| c.len()).max().unwrap();
    for mut line in rows {
        for _ in 0..map_width - line.len() {
            line.push(' ');
        }
        for  c in line.chars() {
            let mut t = Tile::new(c);
            if c == '|' {
                t.velocity = Vec2::new(0., -SPEED);
                t.looping = true
            }
            if c == '~' {
                t.velocity = Vec2::new(SPEED, 0.);
                t.looping = true
            }

            map.push(t);
        }
    }

    println!("Map dimensions : {},{}", map_width, map_height);
    (map, *map_width, map_height)
}
