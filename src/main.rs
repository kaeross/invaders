use crossterm::{
    cursor::{Hide, Show},
    event,
    event::{Event, KeyCode},
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame, Frame},
    render,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{channel, Receiver, Sender},
    thread::JoinHandle,
    time::Duration,
};
use std::{io, thread};

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

    // Render loop
    let (sender, receiver): (Sender<Frame>, Receiver<Frame>) = channel();
    let render_handle = render_handle(receiver);

    // Game loop
    'gameloop: loop {
        // Per frame init
        let curr_frame = new_frame();

        // Input
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

        // Draw & render
        let _ = sender.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

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
