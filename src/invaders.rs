use rusty_time::timer::Timer;
use crate::{ NUM_COLS, NUM_ROWS, frame::Drawable, frame::Frame };
use std::time::Duration;
use std::cmp::max;

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();

        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1) // don't want invader placed all the way to the left
                    && (x < NUM_COLS - 2) // don't place invader to the far right
                    && (y > 0) // don't position invader beyond top of screen
                    && (y < 9) // magic number 9 sets vertical limit
                    && (x % 2 == 0)
                    && (y % 2 == 0) { // modulo to space between each invader, only on even cols and rows
                        
                        army.push(Invader { x, y }); // verbose syntax here instead of new as only instantiated here


                    }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1, 
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();

            let mut downwards = false;

            if self.direction == -1 { // if direction is left

                // iter on vec - immutable iterator
                let min_x = self.army.iter().map(|invader| {invader.x}).min().unwrap_or(0);

                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }

            } else {
                let max_x = self.army.iter().map(|invader| { invader.x }).max().unwrap_or(0);

                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                // increase the speed of army each time moved downwards
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);

                for invader in self.army.iter_mut() {
                    invader.y += 1; // move downwards
                }

            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }

            return true;
        }

        false
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32() > 0.5 {
                "x"
            } else {
                "+"
            };
        }
    }
}