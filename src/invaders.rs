use rusty_time::timer::Timer;
use crate::{ NUM_COLS, NUM_ROWS };

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
}