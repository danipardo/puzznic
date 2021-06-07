use game::levels;
pub mod game;

#[macroquad::main("Puzznic")]
async fn main() {

    let (map, width, height) = levels::load_level(104);

    let mut game_state = game::game_logic::GameLogic::new().await;
    game_state.set_level(map, width, height).await;
    game::play_level(&mut game_state).await;
   
 }

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::game::levels;

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

    #[test]
    fn load_level1(){

        levels::load_level(1);

    }
}
