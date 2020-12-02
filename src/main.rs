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
    - - - - - - - x";
//     let level1 = 
//    "x - - - - - - -
//     - - x x x x x -
//     - x x x x G x -
//     - x x x x x X -
//     - x x x B P C -
//     - x x C P C D -
//     - x E C C C - -
//     - - - - - - - x";

    let level1 = level1.replace(" ", "");
    let mut tilemap = game::TileMap::new(&level1).await;

    tilemap.draw_map(&tilemap.map);

    game::play_level(&mut tilemap).await;

    // for x in tilemap.map{
    //     for title in x {
    //       let c = title.c;
    //      print!("{}", c);
    //   }
    //    println!("");
    //}
}
