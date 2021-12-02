use macroquad::audio::{AudioContext, Sound};

pub enum Sounds {
    MOVE,
    LevelIntro,
}

pub struct Mixer<'a> {
    level_intro: &'a [u8],
    player_move: &'a [u8],
}
impl Mixer<'_> {
    pub fn new() -> Self {
        Mixer {
            level_intro: include_bytes!("../../sound/ogg/1 - Track 1.ogg"),
            player_move: include_bytes!("../../sound/ogg/SFX 1.ogg"),
        }
    }
    pub fn play_sound(&mut self, snd: Sounds) {
        let mut ctx = AudioContext::new();

        match snd {
            Sounds::MOVE => {
                //   let mut sound =macroquad::audio::load_sound(&mut ctx, self.level_intro).await.unwrap();
                //                sound.play(&mut ctx, Default::default());
            }
            Sounds::LevelIntro => {}
        }
    }
}
