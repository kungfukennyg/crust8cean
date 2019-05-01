use ears::{Sound, AudioController};

pub fn play_tone() {
    let mut sound = Sound::new("tone.wav").unwrap();
    sound.play();
    while sound.is_playing() { }
}