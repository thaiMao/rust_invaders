use rusty_time::timer::Timer;
use std::time::Duration;

struct Shot {
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
}