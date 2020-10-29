mod game;

#[macroquad::main("Puzznic")]
async fn main() {
    let level1 = 
   "x - - - - - - -
    - - x x x x x -
    - x x x x x E -
    - x x x x G X -
    - x x x B P C -
    - x x G P C D -
    - x E B X D - -
    - - - - - - - -
  ";

    let level1 = level1.replace(" ", "");
    let tilemap = game::TileMap::new(&level1).await;

    game::play_level(&tilemap).await;

    // for x in tilemap.map{
    //     for title in x {
    //       let c = title.c;
    //      print!("{}", c);
    //   }
    //    println!("");
    //}
}
