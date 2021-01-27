use std::io::Stdout;
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::{ SetBackgroundColor, Color }; 
use crossterm::terminal::{Clear, ClearType };

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        // clear the entire screen

        // queue up a bunch of commands to the terminal and flush
        // them all at once
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
}