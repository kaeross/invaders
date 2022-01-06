use std::{error::Error, sync::mpsc::Sender, thread, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use rusty_audio::Audio;

use crate::{
    audio::{play_game_sound, Sounds},
    frame::{new_frame, Frame},
};

pub fn game_loop(audio: &mut Audio, sender: &Sender<Frame>) -> Result<(), Box<dyn Error>> {
    'gameloop: loop {
        // Per frame init
        let curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        play_game_sound(audio, Sounds::Lose);
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

    Ok(())
}
