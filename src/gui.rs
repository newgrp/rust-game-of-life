#![allow(dead_code)]

/*

This file is responsible for rendering the grid 
to the screen using Piston.

It is also responsible for UI and keyboard controls
for interacting with the display.

*/



use common::LifeAlgorithm;
use piston_window::*;

pub struct GUI {
	paused:bool,
	zoom:f64,
	offset_x:f64,
	offset_y:f64,
	prev_offset_x:f64,
	prev_offset_y:f64,
	mouse_pos:[f64;2],
	mouse_last_pos:[f64;2],
	mouse_middle_down:bool,
}

impl GUI {
	pub fn new() -> GUI {
		GUI {
			paused:false,zoom:1.0,offset_x:0.0,offset_y:0.0,prev_offset_x:0.0,prev_offset_y:0.0,
			mouse_pos:[0.0,0.0],mouse_last_pos:[0.0,0.0],mouse_middle_down:false
		}
	}
	pub fn is_paused(&self) -> bool { self.paused }

	pub fn key_press(&mut self, key:Key){
		if key == Key::Space {
			self.paused = !self.paused;
		}
	}
	
	pub fn mouse_press(&mut self, mouse_btn:MouseButton, life_obj:&mut Box<LifeAlgorithm>, window:&mut PistonWindow){
		let w_size = window.size();
		let window_width = w_size.width; 
		let window_height = w_size.height;

		let x = (((self.mouse_pos[0] - self.offset_x) - (window_width as f64/2.0)) / self.zoom).floor() as isize;
		let y = (((self.mouse_pos[1] - self.offset_y) - (window_height as f64/2.0)) / self.zoom).floor() as isize;

		if mouse_btn == MouseButton::Middle {
			self.mouse_middle_down=true;
		}
		if mouse_btn == MouseButton::Left {
			// Set Alive
			life_obj.set((x,y),true); 
		}
		if mouse_btn == MouseButton::Right {
			// Set Dead
			life_obj.set((x,y),false);
		}
	}
	pub fn mouse_release(&mut self,mouse_btn:MouseButton){
		if mouse_btn == MouseButton::Middle {
			//Stop moving 
			self.mouse_middle_down = false;
		}
	}
	pub fn mouse_move(&mut self,mot:[f64;2]){
		self.mouse_pos =  mot;
		if self.mouse_middle_down == false {
			self.mouse_last_pos = mot;
			self.prev_offset_x = self.offset_x;
			self.prev_offset_y = self.offset_y;
		}
		if self.mouse_middle_down == true {
			self.offset_x = mot[0] - self.mouse_last_pos[0] + self.prev_offset_x;
			self.offset_y = mot[1] - self.mouse_last_pos[1] + self.prev_offset_y;
		}
	}
	pub fn mouse_scroll(&mut self,scroll:[f64;2]){
		let zoom_power = 0.1;
		if scroll[1] == 1.0 {
			self.zoom += zoom_power;
		}
		if scroll[1] == -1.0 {
			self.zoom -= zoom_power;
		}
	}

	pub fn display_ascii(&self,life_obj:&Box<LifeAlgorithm>){
		// Given any object that implements LifeAlgorithm, will display the grid in the terminal
		let bounds = life_obj.get_bounds();
	    let cells = life_obj.output();

		let mut y = bounds.y_max;
        let mut x: isize;
        let mut line: String;
        println!("Generation {}", life_obj.get_generation());
        while y >= bounds.y_min {
            line = "".to_string();
            x = bounds.x_min;
            while x <= bounds.x_max {
                if cells.contains_key(&(x,y)) {
                    if cells[&(x,y)] == true {
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
	
	pub fn draw(&self,life_obj:&Box<LifeAlgorithm>,window:&mut PistonWindow, e:&Event){
		// Given any object that implements LifeAlgorithm, will draw the grid to the screen
		let w_size = window.size();
		let window_width = w_size.width; 
		let window_height = w_size.height;

		window.draw_2d(e, |c, g| {
			clear([1.0, 1.0, 1.0, 1.0], g);

			let half_width:f64 = (window_width as f64)/2.0;
	        let half_height:f64 = (window_height as f64)/2.0;
	        let transform = c.transform.trans(self.offset_x,self.offset_y)
	                                   .trans(half_width,half_height)
	                                   .zoom(self.zoom)
	                                   .trans(-half_width,-half_height);

	        // Get the output to draw from the life object 
	        let bounds = life_obj.get_bounds();
	        let cells = life_obj.output();

	        let mut y = bounds.y_max;
            let mut x: isize;

            while y >= bounds.y_min {
                x = bounds.x_min;
                while x <= bounds.x_max {
                    if cells.contains_key(&(x,y)) {
                        if cells[&(x,y)] == true {
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
}



