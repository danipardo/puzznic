use super::{
    game_logic::PlayingState,
    levels,
    menu_state::{MenuState}
};
use crate::game::sound::Mixer;
use async_trait::async_trait;

#[derive(PartialEq)]
pub enum StateType {
    Menu,
    Playing(usize),
    ExitConfirm,
    Help,
    Quit,
}

pub struct GameState {
    pub state: StateType,
}

#[async_trait]
pub trait Playable {
    async fn run(&mut self, mixer: &mut Mixer) -> StateType;
}

impl GameState {
    pub async fn run(&self, mixer: &mut Mixer) -> StateType {
        match self.state {
            StateType::Menu => {
                let mut menu = MenuState::new().await;
                println!("Jumping to menu");
//                let mut mixer = Mixer::new().await;

                mixer.stop_music();
                return  menu.run(mixer).await;
            }
            StateType::Playing(level) => {
                // Start the game
                let level_info = levels::load_level(level);

                let mut game = PlayingState::new().await;

                game.set_level(level_info).await;
                return game.run(mixer).await;

                
            }
            _ => StateType::Quit,
        }
    }

    pub fn new(state: StateType) -> Self {
        GameState { state }
    }
}
