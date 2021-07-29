use macroquad::prelude::*;
use async_trait::async_trait;

use super::states::Playable;

pub struct MenuState;



impl MenuState {

    pub fn new() -> Self{

        MenuState{

        }
    }
}
#[async_trait]
impl Playable for MenuState {
     async fn run(&self) -> super::states::StateType {

        while !is_key_down(KeyCode::Escape) {
            clear_background(RED);
            next_frame().await;
        }
        
        super::states::StateType::Playing(1)

    }
}