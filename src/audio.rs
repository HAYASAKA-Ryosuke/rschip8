use macroquad::audio::{Sound, play_sound_once, load_sound};

pub struct Audio {
    beep_sound: Sound
}


impl Audio {
    pub async fn new(audio_file_path: &str) -> Audio {
        Audio {
            beep_sound: load_sound(audio_file_path).await.unwrap()
        }
    }

    pub fn play(&mut self) {
        play_sound_once(&self.beep_sound);
    }
}
