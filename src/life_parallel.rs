const NUM_THREADS: usize = 64;

extern crate rand;
extern crate piston_window;

use std::cmp::{min,max};
use std::sync::Arc;
use std::collections::HashSet;
use std::collections::HashMap;
use std::thread;
use std;

use piston_window::*;

#[derive(Clone)]
struct Bounds {   
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Bounds {
    fn new() -> Bounds {
        Bounds { x_min: 0, x_max: 0, y_min: 0, y_max: 0 }
    }

    fn update_bounds(&mut self, x:isize, y:isize) {
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

    fn merge(&mut self, other:Bounds) {
        self.x_min = min(self.x_min, other.x_min);
        self.x_max = max(self.x_max, other.x_max);
        self.y_min = min(self.y_min, other.y_min);
        self.y_max = max(self.y_max, other.y_max);
    }
}

fn cells_access_record(s:&str) {
    println!("Arc::get_mut(&mut self.cells) returned None at {}", s);
}

// fn parts_access_record(k:usize, s:&str) {
//     println!("Arc::get_mut(&mut self.parts[{}]) returned None at {}", k, s);
// }

#[derive(Clone)]
pub struct Life {
    pub generation: i64,
    pub cells: Arc<HashMap<(isize, isize), i8>>,
    parts: Vec<Arc<HashSet<(isize, isize)>>>,
    rect: Bounds,
    window_width:u32,
    window_height:u32,
}

impl Life {
    pub fn new(width:u32,height:u32) -> Life {
        Life { generation: 0, cells: Arc::new(HashMap::new()), parts: vec![Arc::new(HashSet::new()); NUM_THREADS], rect: Bounds::new(), window_width:width,window_height:height }
    }

    pub fn set_primitive(&mut self, t:(isize,isize), v: i8) {
        let x = t.0;
        let y = t.1;
        if !self.cells.contains_key(&(x,y)) {
            if let Some(re) = Arc::get_mut(&mut self.cells) {
                (*re).insert((x,y), v);
            } else {
                cells_access_record("Life::set, does not contain key");
            }
            let ind = rand::random::<usize>()%NUM_THREADS;
            (*Arc::make_mut(&mut self.parts[ind])).insert((x,y));
            // if let Some(pe) = Arc::get_mut(&mut self.parts[ind]) {
            //     (*pe).insert((x,y));
            // } else {
            //     parts_access_record(ind, "Life::set, does not contain_key");
            // }
        }
        else {
            if let Some(re) = Arc::get_mut(&mut self.cells) {
                if let Some(z) = (*re).get_mut(&(x,y)) {
                    *z = v;
                }
            } else {
                cells_access_record("Life::set, contains key");
            }
        }
    }

    pub fn set(&mut self, t:(isize,isize), v: i8) {
        self.set_primitive(t,v);
        self.cleanup();
    }

    fn get_adjacent(x:isize, y:isize) -> Vec<(isize, isize)> {
        vec![(x+1, y  ),
             (x+1, y+1),
             (x  , y+1),
             (x-1, y+1),
             (x-1, y  ),
             (x-1, y-1),
             (x  , y-1),
             (x+1, y-1)]
    }

    // fn next_val(&self, x:isize, y:isize) -> i8 {
    //     let mut neighbors: i8 = 0;
    //     for (i,j) in Life::get_adjacent(x,y) {
    //         if self.cells.contains_key(&(i,j)) {
    //             neighbors += self.cells[&(i,j)];
    //         }
    //     }
    //     if (neighbors == 3) | ((neighbors == 2) & (self.cells[&(x,y)] == 1)) {
    //         1
    //     } else {
    //         0
    //     }
    // }

    fn next_val_from_arc(cells_ref:&Arc<HashMap<(isize, isize), i8>>, x:isize, y:isize) -> i8 {
        let mut neighbors: i8 = 0;
        for (i,j) in Life::get_adjacent(x,y) {
            if cells_ref.contains_key(&(i,j)) {
                neighbors += cells_ref[&(i,j)];
            }
        }
        if (neighbors == 3) | ((neighbors == 2) & (cells_ref[&(x,y)] == 1)) {
            1
        } else {
            0
        }
    }

    pub fn cleanup(&mut self) {
        self.rect.x_min = std::isize::MAX;
        self.rect.x_max = std::isize::MIN;
        self.rect.y_min = std::isize::MAX;
        self.rect.y_max = std::isize::MIN;
        let mut thread_handles = vec![];
        for k in 0..NUM_THREADS {
            let my_cells = self.cells.clone();
            let my_part = self.parts[k].clone();
            let mut temp = self.rect.clone();
            thread_handles.push(thread::spawn(move || {
                let mut to_add: Vec<(isize, isize)> = vec![];
                let mut to_del: Vec<(isize, isize)> = vec![];
                for &(x,y) in my_part.iter() {
                    if my_cells[&(x,y)] == 1 {
                        temp.update_bounds(x,y);
                        for (i,j) in Life::get_adjacent(x,y) {
                            if !my_cells.contains_key(&(i,j)) {
                                to_add.push((i,j));
                            }
                        }
                    } else {
                        let mut barren = true;
                        for (i,j) in Life::get_adjacent(x,y) {
                            if my_cells.contains_key(&(i,j)) {
                                if my_cells[&(i,j)] == 1 {
                                    barren = false;
                                    break;
                                }
                            }
                        }
                        if barren {
                            to_del.push((x,y));
                        } else {
                            temp.update_bounds(x,y);
                        }
                    }
                }
                (temp, to_add, to_del)
            }));
        }
        let mut to_adds: Vec<Vec<(isize, isize)>> = vec![];
        let mut to_dels: Vec<Vec<(isize, isize)>> = vec![];
        for hand in thread_handles {
            let ret = hand.join().unwrap();
            self.rect.merge(ret.0);
            to_adds.push(ret.1);
            to_dels.push(ret.2);
        }
        for k in 0..NUM_THREADS {
            for &(x,y) in &to_adds[k] {
                if !self.cells.contains_key(&(x,y)) {
                    if let Some(re) = Arc::get_mut(&mut self.cells) {
                        (*re).insert((x,y), 0);
                    } else {
                        cells_access_record("Life::cleanup, inserting new cells");
                    }
                    self.rect.update_bounds(x,y);
                    let ind = rand::random::<usize>()%NUM_THREADS;
                    (*Arc::make_mut(&mut self.parts[ind])).insert((x,y));
                    // if let Some(pe) = Arc::get_mut(&mut self.parts[ind]) {
                    //     (*pe).insert((x,y));
                    // } else {
                    //     parts_access_record(ind, "Life::cleanup, inserting new cells");
                    // }
                }
            }
            for &(x,y) in &to_dels[k] {
                if let Some(re) = Arc::get_mut(&mut self.cells) {
                    (*re).remove(&(x,y));
                } else {
                    cells_access_record("Life::cleanup, removing cells");
                }
                (*Arc::make_mut(&mut self.parts[k])).remove(&(x,y));
                // if let Some(pe) = Arc::get_mut(&mut self.parts[k]) {
                //     (*pe).remove(&(x,y));
                // } else {
                //     parts_access_record(k, "Life::cleanup, removing cells");
                // }
            }
        }
    }

    pub fn advance(&mut self) {
        let mut thread_handles = vec![];
        for k in 0..NUM_THREADS {
            let my_cells = self.cells.clone();
            let my_part = self.parts[k].clone();
            thread_handles.push(thread::spawn(move || {
                let mut cells_new = HashMap::new();
                for &(x,y) in my_part.iter() {
                    cells_new.insert((x,y), Life::next_val_from_arc(&my_cells,x,y));
                }
                cells_new
            }));
        }
        let mut cells_new: Vec<HashMap<(isize, isize), i8>> = vec![];
        for hand in thread_handles {
            cells_new.push(hand.join().unwrap());
        }
        for k in 0..NUM_THREADS {
            for (&(x,y),v) in &cells_new[k] {
                if let Some(re) = Arc::get_mut(&mut self.cells) {
                    if let Some(z) = (*re).get_mut(&(x,y)) {
                        *z = *v;
                    }
                } else {
                    cells_access_record("Life::advance");
                }
            }
        }
        self.cleanup();
        self.generation += 1;
    }

    pub fn draw(&self, window:&mut PistonWindow, e:&Event,zoom:f64,offset_x:f64,offset_y:f64) {
 
        window.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
 
            let half_width:f64 = (self.window_width as f64)/2.0;
            let half_height:f64 = (self.window_height as f64)/2.0;
            let transform = c.transform.trans(offset_x,offset_y)
                                       .trans(half_width,half_height)
                                       .zoom(zoom)
                                       .trans(-half_width,-half_height);
 
            let mut y = self.rect.y_max;
            let mut x: isize;
 
            while y >= self.rect.y_min {
 
                x = self.rect.x_min;
                while x <= self.rect.x_max {
                    if self.cells.contains_key(&(x,y)) {
                        if self.cells[&(x,y)] == 1 {
                            // Alive 
                            rectangle([1.0, 0.0, 0.0, 1.0], // red
                                       [x as f64 + half_width, y as f64 + half_height, 1.0 ,1.0], // rectangle
                                       transform, g);
                            
                        } else {
                            // Dead
 
                        } 
                    } else {
                        // Dead
 
                    }
                    x += 1;
                }
                
                y -= 1;
            }
            c.reset();
        });
 
    }

    pub fn display(&self) {
        let mut y = self.rect.y_max;
        let mut x: isize;
        let mut line: String;
        println!("generation {}", self.generation);
        while y >= self.rect.y_min {
            line = "".to_string();
            x = self.rect.x_min;
            while x <= self.rect.x_max {
                if self.cells.contains_key(&(x,y)) {
                    if self.cells[&(x,y)] == 1 {
                        line.push_str("*");
                    } else {
                        // let mut n:usize = 233;
                        // for k in 0..NUM_THREADS {
                        //     if self.parts[k].contains(&(x,y)) {
                        //         n = k;
                        //     }
                        // }
                        // if n < 10 {
                        //     line += &format!("0{}", n);
                        // } else {
                        //     line += &format!("{}", n);
                        // }
                        line.push_str(" ");
                    }
                } else {
                    line.push_str(" ");
                }
                x += 1;
            }
            println!("{}", line);
            y -= 1;
        }
        println!("\n");
    }
}
