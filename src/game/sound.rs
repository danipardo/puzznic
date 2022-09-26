

use macroquad::audio::{Sound, PlaySoundParams};

pub enum Sounds {
    MOVE,
    LevelIntro,
    Playing,
    Collided,
}

pub struct Mixer {
    level_intro: Sound,
    player_move: Sound,
    playing: Sound,
    collided: Sound,
}
impl Mixer {
    pub async fn new() -> Self {
        Mixer {
            level_intro: macroquad::audio::load_sound("sound/ogg/1 - Track 1.ogg")
                .await
                .unwrap(),
            playing: macroquad::audio::load_sound("sound/ogg/2 - Track 2.ogg")
                .await
                .unwrap(),
            player_move: macroquad::audio::load_sound("sound/ogg/SFX 2.ogg")
                .await
                .unwrap(),
            collided: macroquad::audio::load_sound("sound/ogg/SFX 17.ogg")
                .await
                .unwrap(),
        }
    }
    pub async fn stop_sound(&mut self, _snd: Sounds) {
        macroquad::audio::stop_sound(self.playing);

    }
    pub async fn play_sound(&mut self, snd: Sounds) {
        // let mut ctx = AudioContext::new();
        match snd {
            Sounds::MOVE => {
                macroquad::audio::play_sound_once(self.player_move);
            }
            Sounds::LevelIntro => {
                macroquad::audio::play_sound_once(self.level_intro);
            }
            Sounds::Playing => {                
                macroquad::audio::play_sound(self.playing, PlaySoundParams{
                    looped: true,
                    volume: 1.,
                });
            }
            Sounds::Collided => {
                macroquad::audio::play_sound_once(self.collided);
            }
        }
    }
}
