use super::{
    game_logic::PlayingState,
    levels,
    menu_state::{MenuState},
};
use async_trait::async_trait;

#[derive(PartialEq)]
pub enum StateType {
    Menu,
    Playing(usize),
    Help,
    Quit,
}

pub struct GameState {
    pub state: StateType,
}

#[async_trait]
pub trait Playable {
    async fn run(&mut self) -> StateType;
}

impl GameState {
    pub async fn run(&self) -> StateType {
        match self.state {
            StateType::Menu => {
                let mut menu = MenuState::new().await;
                return menu.run().await;
            }
            StateType::Playing(level) => {
                // Start the game
                let level_info = levels::load_level(level);

                let mut game = PlayingState::new().await;

                game.set_level(level_info).await;
                return game.run().await;

                //                play_level(&mut game_state).await;
            }
            _ => StateType::Quit,
        }
    }

    pub fn new(state: StateType) -> Self {
        GameState { state }
    }
}
