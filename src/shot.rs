use rusty_time::timer::Timer;
use std::time::Duration;
use crate::frame::{ Drawable, Frame };

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50), // a shot will move upwards by one cell in 50ms
        }
    }


    pub fn update(&mut self, delta: Duration) {

        // update timer to make timer start counting down by delta amount
        self.timer.update(delta);

        // only move if timer is ready and not exploding
        if self.timer.ready && !self.exploding {
            // if we haven't reached the top of the screen, we can move upwards
            if self.y > 0 {
                self.y -= 1;
            }

            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    // draw the shot
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}