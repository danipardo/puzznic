

use async_trait::async_trait;
use macroquad::prelude::*;

use super::states::{Playable, StateType};

pub struct MenuState {
    font: Font,
    selection: u8,
}

impl MenuState {
    pub async fn new() -> Self {
        let font = load_ttf_font("Nintendo-NES-Font.ttf").await.unwrap();

        MenuState { selection: 0, font }
    }
    fn get_text_params(&self, selection: u8) -> TextParams {
        let (fs, fc, fa) = camera_font_scale(8.);

        let mut tp = TextParams {
            font: self.font,
            font_size: fs,
            font_scale: fc,
            font_scale_aspect: fa,
            color: LIGHTGRAY,
        };

        if self.selection == selection {
            tp.color = WHITE;
        }
        tp
    }
}
#[async_trait]
impl Playable for MenuState {
    async fn run(&mut self) -> StateType {
        let desired_ratio = 320. / 200. as f32;

        let background = load_texture("img/menu_bg.png").await.unwrap();
        background.set_filter(FilterMode::Nearest);
        loop {
            let physical_ratio = screen_width() / screen_height();

            let mut w = 320.;
            let mut h = 200.;
            if physical_ratio / desired_ratio > 1. {
                w = 320. * physical_ratio / desired_ratio;
            }
            if physical_ratio / desired_ratio < 1. {
                h = 200. * desired_ratio / physical_ratio;
            }
            let camera = Camera2D::from_display_rect(Rect::new(0., 0., w, h));

            set_camera(&camera);

            let (fs, fc, fa) = camera_font_scale(10.);
            let tp = TextParams {
                font: self.font,
                font_size: fs,
                font_scale: fc,
                font_scale_aspect: fa,
                color: GREEN,
            };

            clear_background(BLACK);
            let bg_params = DrawTextureParams {
                dest_size: Some(Vec2::new(320., 200.)),
                source: Some(Rect::new(0., 0., 320., 200.)),
                rotation: 0.,
                pivot: None,
                flip_x: false,
                flip_y: false,
            };

            draw_texture_ex(background, 0., 0., WHITE, bg_params);

            draw_text_ex("PUZZNIC!", 110., 80., tp);
            draw_text_ex("NEW GAME", 100., 120., self.get_text_params(0));
            draw_text_ex("INSTRUCTIONS", 100., 140., self.get_text_params(1));
            draw_text_ex("QUIT", 100., 160., self.get_text_params(2));
            next_frame().await;
            if is_key_pressed(KeyCode::Down) {
                self.selection += 1;
                if self.selection == 3 {
                    self.selection = 0;
                }
            }
            if is_key_pressed(KeyCode::Up) {
                if self.selection == 0 {
                    self.selection = 2;
                } else {
                    self.selection -= 1;
                }
            }
            if is_key_pressed(KeyCode::Enter) {
                match self.selection {
                    0 => return StateType::Playing(1),
                    1 => return StateType::Help,
                    2 => return StateType::Quit,
                    _ => {}
                }
            }
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
        }

        StateType::Playing(1)
    }
}
