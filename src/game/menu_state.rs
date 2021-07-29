use async_trait::async_trait;
use macroquad::prelude::*;

use super::states::{Playable, StateType};

pub struct MenuState;

impl MenuState {
    pub fn new() -> Self {
        MenuState {}
    }
}
#[async_trait]
impl Playable for MenuState {
    async fn run(&mut self) -> StateType {
        loop {
            clear_background(RED);
            next_frame().await;
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
        }

        StateType::Playing(1)
    }
}
