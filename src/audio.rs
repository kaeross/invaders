use rusty_audio::Audio;

pub enum Sounds {
    Explode,
    r#Move,
    Pew,
    Startup,
    Win,
    Lose,
}

pub fn load_audio() -> Audio {
    let mut audio = Audio::new();

    audio.add("explode", "sounds/explode.mp3");
    audio.add("lose", "sounds/lose.mp3");
    audio.add("move", "sounds/move.mp3");
    audio.add("pew", "sounds/pew.mp3");
    audio.add("startup", "sounds/startup.mp3");
    audio.add("win", "sounds/win.mp3");

    audio
}

pub fn play_game_sound(audio: &mut Audio, sound: Sounds) {
    match sound {
        Sounds::Explode => audio.play("explode"),
        Sounds::Move => audio.play("move"),
        Sounds::Pew => audio.play("pew"),
        Sounds::Startup => audio.play("startup"),
        Sounds::Win => audio.play("win"),
        Sounds::Lose => audio.play("lose"),
    };
}
