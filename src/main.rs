use crossterm::cursor::{Hide, Show};
use crossterm::{
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.mp3");
    audio.add("lose", "sounds/lose.mp3");
    audio.add("move", "sounds/move.mp3");
    audio.add("pew", "sounds/pew.mp3");
    audio.add("startup", "sounds/startup.mp3");
    audio.add("win", "sounds/win.mp3");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
