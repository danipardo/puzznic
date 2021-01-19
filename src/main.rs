pub mod game;

#[macroquad::main("Puzznic")]
async fn main() {
    let level1 = "x - - - - - - -
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
    let mut tilemap = game::game_state::GameState::new(&level1).await;

    // tilemap.draw_map(&tilemap.map);

    game::play_level(&mut tilemap).await;
   
    // for x in tilemap.map{
    //     for title in x {
    //       let c = title.c;
    //      print!("{}", c);
    //   }
    //    println!("");
    //}
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;


    #[test]
    fn test1() {
        pub struct A {
            content: RefCell<String>,
        }

        impl A {
            pub fn new(a: &str) -> Self {
                Self {
                    content: RefCell::new(String::from(a)),
                }
            }
            pub fn modify_content(&self) {
                self.content.replace(String::from("modified content"));
            }
        }

        let a = A::new("xxx");

        a.modify_content();
        assert_eq!(*a.content.borrow(), String::from("modified content"));
    }
}
