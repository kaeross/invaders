use std::{
    error::Error,
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use rusty_audio::Audio;

use crate::{
    audio::{play_game_sound, Sounds},
    frame::{new_frame, Drawable, Frame},
    invaders::Invaders,
    player::Player,
};

pub fn game_loop(audio: &mut Audio, sender: &Sender<Frame<str>>) -> Result<(), Box<dyn Error>> {
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            play_game_sound(audio, Sounds::Pew);
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        play_game_sound(audio, Sounds::Lose);
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);

        if player.detect_hits(&mut invaders) {
            play_game_sound(audio, Sounds::Explode);
        }

        if invaders.update(delta) {
            play_game_sound(audio, Sounds::Move);
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];

        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = sender.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose
        if invaders.all_killed() {
            play_game_sound(audio, Sounds::Win);
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            play_game_sound(audio, Sounds::Lose);
            break 'gameloop;
        }
    }

    Ok(())
}
