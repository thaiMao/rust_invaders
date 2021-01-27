use std::{ io };
use std::error::Error;
use std::time::{ Duration };
use rusty_audio::Audio;
use crossterm::{ terminal, ExecutableCommand, event };
use crossterm::event::{Event};
use crossterm::terminal::{ EnterAlternateScreen, LeaveAlternateScreen };
use crossterm::cursor::{ Hide, Show };


fn main() -> Result<(), Box<dyn Error>>  {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game Loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {

            }

        }
    }

    // Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
