use crossterm::{
    cursor::{Hide, Show},
    event,
    event::{Event, KeyCode},
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::io;
use std::{error::Error, time::Duration};

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
    let stdout = init_terminal()?;

    // Game loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    cleanup(audio, stdout)
}

fn init_terminal() -> Result<std::io::Stdout, Box<dyn Error>> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;

    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    Ok(stdout)
}

fn cleanup(audio: rusty_audio::Audio, mut stdout: std::io::Stdout) -> Result<(), Box<dyn Error>> {
    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    Ok(())
}
