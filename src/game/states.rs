use super::{game_logic::GameLogic, levels, menu_state::{self, MenuState}, play_level};
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
   async fn run(&self) -> StateType;
}

impl GameState {
    pub async fn run(&self) -> StateType {
        match self.state {
            StateType::Menu => {
                let menu = MenuState::new();
                return menu.run().await;
            },
            StateType::Playing(level) => {

                // Start the game
                let (map, width, height) = levels::load_level(82);

                let mut game_state = GameLogic::new().await;
                game_state.set_level(map, width, height).await;
                play_level(&mut game_state).await;
            
                return StateType::Menu
            },
            _ => StateType::Quit,
        }
    }

    pub fn new(state: StateType) -> Self {
        GameState { state }
    }
}
