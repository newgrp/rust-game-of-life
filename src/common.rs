/*

This file defines the trait that all game-of-life algorithms
must implement. It's mainly so that we can swap out 
different game-of-life algorithms without the rest 
of our code having to be aware.

It also defines the Bounds struct that keeps track of 
the size of the map.

*/

use std::collections::HashMap;



pub trait LifeAlgorithm {
	fn advance_by(&mut self,count:u64); //Advances the simulation forward [count] step(s) 
	fn set(&mut self,cell:(isize,isize), value: i8); //Sets the value (0 or 1, dead or alive) of a given cell (x,y)
	fn clean_up(&mut self); //Performs any necessary clean up after setting values (for resizing the hashmap) 
	fn clear(&mut self); //Clears the entire grid
	fn get_generation(&self) -> i64; //Get the current generation
	fn get_bounds(&self) -> Bounds; // Gets the bounds of this life simulation
	fn output(&self) -> &HashMap<(isize, isize), i8>; // Get immutable reference to hashmap of (x,y) points, and 0 (dead) or 1 (alive). Used to draw on screen or output as ASCII in terminal
}

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

    pub fn update_bounds(&mut self, x:isize, y:isize) {
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
}