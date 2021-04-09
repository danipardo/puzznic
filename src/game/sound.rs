
use quad_snd::{
    decoder::{read_ogg, read_wav_ext},
    mixer::{PlaybackStyle, SoundMixer},
};

pub enum Sound {
    MOVE,
    LevelIntro,
}

pub struct Mixer<'a> {
    mixer: SoundMixer,
    level_intro: &'a [u8],
    player_move: &'a [u8],

}
impl Mixer<'_> {
    pub fn new() -> Self {
        Mixer {
            mixer: SoundMixer::new(),
            level_intro: include_bytes!("../../sound/ogg/1 - Track 1.ogg"),
            player_move: include_bytes!("../../sound/ogg/SFX 1.ogg"),
        }
    }
    pub fn play_sound(&mut self, snd: Sound) {
        

        match snd {
            Sound::MOVE => {
                let player_move = read_ogg(self.player_move).unwrap();
                self.mixer.play(player_move);
            }
            Sound::LevelIntro => {
                let level_intro = read_ogg(self.level_intro).unwrap();
                self.mixer.play(level_intro);

            }
        }
    }
}
