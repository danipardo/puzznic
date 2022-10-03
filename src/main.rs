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

    let mut mixer = game::sound::Mixer::new().await;
    while state.state != StateType::Quit {
        let new_state = state.run(&mut mixer).await;
        state = GameState::new(new_state);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::levels;
    #[test]
    fn load_level1() {
        for i in 1..161 {
            println!("Testing level {i}");
            levels::load_level(i);
        }

    }
}
