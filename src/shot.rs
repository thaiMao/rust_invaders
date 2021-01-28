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
        if self.timer.ready && !self.exploding {

        }
    }
}