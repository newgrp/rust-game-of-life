/*

This file defines the trait that all game-of-life algorithms
must implement. It's mainly so that we can swap out 
different game-of-life algorithms without the rest 
of our code having to be aware.

It also defines the Bounds struct that keeps track of 
the size of the map.

*/

use std::iter::Iterator;

pub trait LifeAlgorithm<I: Iterator<Item=(isize, isize)>> {
	/// Advances the simulation forward [count] step(s) 
    fn advance_by(&mut self,count:u64);
	
    /// Sets the value (false or true, dead or alive) of a given cell (x,y)
    fn set(&mut self, cell: (isize, isize), value: bool);
	
    /// Performs any necessary clean up after setting values (for resizing the hashmap) 
    fn clean_up(&mut self);
	
    /// Clears the entire grid
    fn clear(&mut self);
	
    /// Get the current generation
    fn get_generation(&self) -> u64;
	
    /// Gets the bounds of this life simulation
    fn get_bounds(&self) -> Bounds;
    
    /// Gets the value of the cell (x,y) as a bool
    fn get_value(&self, cell: (isize, isize)) -> bool;
    
    /// Gets an iterator over all the live cells. Used to draw on screen or output as ASCII in terminal
	fn live_cells(&self) -> I;
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

    pub fn from_half_side(s: isize) -> Bounds {
        assert!(s >= 0);
        Bounds { x_min: -s,
                 x_max:  s,
                 y_min: -s,
                 y_max:  s }
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
