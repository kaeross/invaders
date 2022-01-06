use crossterm::{
    cursor::{Hide, Show},
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    audio::{load_audio, play_game_sound, Sounds},
    frame::{self, Frame},
    game, render,
};

use std::{
    error::Error,
    sync::mpsc::{channel, Receiver, Sender},
    thread::JoinHandle,
};
use std::{io, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = load_audio();
    play_game_sound(&mut audio, Sounds::Startup);

    // Terminal
    let stdout = init_terminal()?;

    // Render loop
    let (sender, receiver): (Sender<Frame>, Receiver<Frame>) = channel();
    let render_handle = render_handle(receiver);

    // Game loop
    game::game_loop(&mut audio, &sender)?;

    // Cleanup
    cleanup(audio, stdout, sender, render_handle)
}

fn init_terminal() -> Result<std::io::Stdout, Box<dyn Error>> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;

    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    Ok(stdout)
}

fn cleanup(
    audio: rusty_audio::Audio,
    mut stdout: std::io::Stdout,
    sender: Sender<Frame>,
    render_handle: JoinHandle<()>,
) -> Result<(), Box<dyn Error>> {
    audio.wait();

    drop(sender);
    render_handle.join().unwrap();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    Ok(())
}

fn render_handle(reciever: Receiver<Frame>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match reciever.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    })
}
