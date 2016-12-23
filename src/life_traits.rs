extern crate piston_window;
extern crate gfx_graphics;

use piston_window::*;

#[derive(Clone)]
pub struct Bounds {   
    pub x_min: isize,
    pub x_max: isize,
    pub y_min: isize,
    pub y_max: isize,
}

impl Bounds {
    pub fn new() -> Bounds {
        Bounds { x_min: 0, x_max: 0, y_min: 0, y_max: 0 }
    }

    pub fn from_half_side(len: isize) -> Bounds {
        Bounds { x_min: len, x_max: len, y_min: len, y_max: len }
    }

    pub fn from_box(left: isize, right: isize, bot: isize, top: isize) -> Bounds {
        Bounds { x_min: left, x_max: right, y_min: bot, y_max: top }
    }

    pub fn contains(self, x: isize, y: isize) -> bool {
        (x >= self.x_min) && (x <= self.x_max) && (y >= self.y_min) && (y <= self.y_max)
    }

    pub fn update_bounds(&mut self, x: isize, y: isize) {
        if x < self.x_min {
            self.x_min = x;
        }
        if x > self.x_max {
            self.x_max = x;
        }
        if y < self.y_min {
            self.y_min = y;
        }
        if y > self.y_max {
            self.y_max = y;
        }
    }

    pub fn outset(&mut self, amt: isize) {
        self.x_min -= amt;
        self.x_max += amt;
        self.y_min -= amt;
        self.y_max += amt;
    }
}

pub trait LifeObject {
    fn from_file(&str) -> Self;
    fn get_generation(&self) -> u64;
    fn get_bounds(&self) -> Bounds;
    fn get_value(&self, isize, isize) -> bool;
    fn set(&mut self, isize, isize, bool);
    fn toggle(&mut self, isize, isize);
    // fn advance_by(&mut self, u64) -> &mut Self;
}

pub trait LifeGUI {
    fn draw(&self, &mut PistonWindow, &Event, f64, f64, f64);
}
