
#[derive(Debug, Copy, Clone)]
pub struct Tile{

    pub c: char

}

pub struct TileMap{

    pub map: Vec<Vec<Tile>>

}

pub fn load_map( level : &String) -> TileMap {


    let mut map: Vec<Vec<Tile>> = vec![];

     let lines = level.split("\n");

     for line in lines {

        let mut s1 = vec![];
        for c in line.chars() {

            let t: Tile = Tile{c};

            s1.push(t);
        }

        map.push(s1);

     }
    TileMap{map}

}