use std::{ io, thread };
use std::sync::mpsc;
use std::error::Error;
use std::time::{ Duration, Instant };
use rusty_audio::Audio;
use crossterm::{ terminal, ExecutableCommand, event };
use crossterm::event::{Event, KeyCode };
use crossterm::terminal::{ EnterAlternateScreen, LeaveAlternateScreen };
use crossterm::cursor::{ Hide, Show };
use rust_invaders::{ frame, render, player::Player, invaders::Invaders };
use rust_invaders::frame::Drawable;
use rust_invaders::frame::new_frame;

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



    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    // Game Loop
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();  // elapsed time since it began 
        instant = Instant::now(); // delta will be the exact time taken to go around the game loop
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    KeyCode::Right => player.move_right(),
                    KeyCode::Left => player.move_left(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);

        if invaders.update(delta) {
            audio.play("move");
        }


        // Draw & render
        player.draw(&mut curr_frame);

        /* let _ is used instead of unwrap() (which would crash the program)
        becuase the gameloop will start before the "render" thread is spawned
        and rather than erroring and crashing a let _ is used to ignore the error
        */ 
        let _ = render_tx.send(curr_frame);

        // game loop is much faster render loop
        thread::sleep(Duration::from_millis(1));

    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
