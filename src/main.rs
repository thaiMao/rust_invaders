use std::{ io, thread };
use std::sync::mpsc;
use std::error::Error;
use std::time::{ Duration };
use rusty_audio::Audio;
use crossterm::{ terminal, ExecutableCommand, event };
use crossterm::event::{Event, KeyCode };
use crossterm::terminal::{ EnterAlternateScreen, LeaveAlternateScreen };
use crossterm::cursor::{ Hide, Show };
use rust_invaders::{ frame, render };

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

    // Render Loop in a separate thread
    // set up a channel to communicate with thread
    // use cross beam channel for higher performance
    // using mpsc channels for demo purposes
    let (render_tx, render_rx) = mpsc::channel();
    // make a thread
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }

        
    });




    // Game Loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
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
