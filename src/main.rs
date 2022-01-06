use rusty_audio::Audio;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.mp3");
    audio.add("lose", "sounds/lose.mp3");
    audio.add("move", "sounds/move.mp3");
    audio.add("pew", "sounds/pew.mp3");
    audio.add("startup", "sounds/startup.mp3");
    audio.add("win", "sounds/win.mp3");

    audio.play("startup");

    // Cleanup
    audio.wait();
    Ok(())
}
