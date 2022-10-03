use game::{
    states::{GameState, StateType},
};
use macroquad::prelude::{Conf};
pub mod game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Puzznic".to_owned(),
        window_width: 320 * 3,
        window_height: 200 * 3,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::new(StateType::Menu);

    while state.state != StateType::Quit {
        let new_state = state.run().await;
        state = GameState::new(new_state);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::levels;
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

    #[test]
    fn load_level1() {
        levels::load_level(3);
    }
}
