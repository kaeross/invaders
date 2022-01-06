use std::{error::Error, sync::mpsc::Sender, thread, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use rusty_audio::Audio;

use crate::{
    audio::{play_game_sound, Sounds},
    frame::{new_frame, Drawable, Frame},
    player::Player,
};

pub fn game_loop(audio: &mut Audio, sender: &Sender<Frame>) -> Result<(), Box<dyn Error>> {
    let mut player = Player::new();

    'gameloop: loop {
        // Per frame init
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        play_game_sound(audio, Sounds::Lose);
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw & render
        player.draw(&mut curr_frame);
        let _ = sender.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
