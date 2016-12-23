extern crate piston_window;
extern crate gfx_graphics;

use std::collections::HashMap;
use std;

use piston_window::*;

use life_traits::Bounds;

pub struct Life {
    pub generation: i64,
    pub cells: HashMap<(isize, isize), i8>,
    rect: Bounds,
    window_width:u32,
    window_height:u32,
}

impl Life {
    pub fn new(width:u32,height:u32) -> Life {
        Life { generation: 0,cells: HashMap::new(), rect: Bounds::new(), window_width:width,window_height:height}
    }

    pub fn set_primitive(&mut self, t:(isize,isize), v: i8) {
        let x = t.0;
        let y = t.1;
        if !self.cells.contains_key(&(x,y)) {
            self.cells.insert((x,y), v);
        }
        else {
            if let Some(z) = self.cells.get_mut(&(x,y)) {
                *z = v;
            }
        }
        
    }

    pub fn set(&mut self, t:(isize,isize), v: i8) {
        self.set_primitive(t,v);
        self.cleanup();
    }

    fn get_adjacent(&self, x:isize, y:isize) -> Vec<(isize, isize)> {
        vec![(x+1, y  ),
             (x+1, y+1),
             (x  , y+1),
             (x-1, y+1),
             (x-1, y  ),
             (x-1, y-1),
             (x  , y-1),
             (x+1, y-1)]
    }

    fn next_val(&self, x:isize, y:isize) -> i8 {
        let mut neighbors: i8 = 0;
        for (i,j) in self.get_adjacent(x,y) {
            if self.cells.contains_key(&(i,j)) {
                neighbors += self.cells[&(i,j)];
            }
        }
        if (neighbors == 3) | ((neighbors == 2) & (self.cells[&(x,y)] == 1)) {
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
        let mut to_add: Vec<(isize, isize)> = vec![];
        let mut to_del: Vec<(isize, isize)> = vec![];
        for (&(x,y),v) in &self.cells {
            if *v == 1 {
                self.rect.update_bounds(x,y);
                for (i,j) in self.get_adjacent(x,y) {
                    if !self.cells.contains_key(&(i,j)) {
                        to_add.push((i,j));
                    }
                }
            } else {
                let mut barren = true;
                for (i,j) in self.get_adjacent(x,y) {
                    if self.cells.contains_key(&(i,j)) {
                        if self.cells[&(i,j)] == 1 {
                            barren = false;
                            break;
                        }
                    }
                }
                if barren {
                    to_del.push((x,y));
                } else {
                    self.rect.update_bounds(x,y);
                }
            }
        }
        for (x,y) in to_add {
            self.cells.insert((x,y), 0);
            self.rect.update_bounds(x,y);
        }
        for (x,y) in to_del {
            self.cells.remove(&(x,y));
        }
    }

    pub fn advance(&mut self) {
        let mut cells_new: HashMap<(isize, isize), i8> = HashMap::new();
        for &(x,y) in self.cells.keys() {
           cells_new.insert((x,y), self.next_val(x,y));
        }
        self.cells = cells_new;
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
            //Reset transform
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
