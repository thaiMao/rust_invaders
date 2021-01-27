use std::io::Stdout;
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::{ SetBackgroundColor, Color }; 
use crossterm::terminal::{Clear, ClearType };
use crossterm::cursor::MoveTo;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        // clear the entire screen

        // queue up a bunch of commands to the terminal and flush
        // them all at once
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    // iterate through entire frame and draw what has changed
    for (x, col) in current_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            // if the character has changed or we are forcing rendering
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }

}