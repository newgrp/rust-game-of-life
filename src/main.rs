extern crate piston_window;
extern crate time;
extern crate gfx_graphics;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

use piston_window::*;

const WINDOW_WIDTH:u32 = 600;
const WINDOW_HEIGHT:u32 = 400;

mod life_traits;
mod life_sequential;
mod life_parallel;
mod hashlife_sequential;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut mode = "single".to_string();
    let mut zoom = 1.0;
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;
    let mut name = "Conrad's Game of Life";
    let mut file_to_load = "src/pento.cells".to_string();

    if args.len() > 1 {
        mode = args[1].parse().unwrap();
    }
    if args.len() > 2 {
        zoom = args[2].parse().unwrap();
    }
    if args.len() > 3 {
        file_to_load = args[3].parse().unwrap();
    }

    let mut r_pentomino = life_sequential::Life::new(WINDOW_WIDTH,WINDOW_HEIGHT);
    let mut r_pentomino_parallel = life_parallel::Life::new(WINDOW_WIDTH,WINDOW_HEIGHT);

    // Read pattern from file 
    let mut cells_file = File::open(file_to_load).unwrap();
    let mut primer_data = String::new();
    
    match cells_file.read_to_string(&mut primer_data){
        Err(why) => panic!("couldn't read file - {}", why.description()),
        Ok(_) => println!("Succesfully read file!"),
    }

    let lines = primer_data.split("\n");
    let mut row = 0;
    let mut column = 0;
    for l in lines {
        if l != "" {
            let first_char = l.chars().nth(0).unwrap();
            // Skip comments 
            if first_char != '!' {
                //This is a data line 
                

                //Iterate over chars 
                for c in l.chars() {
                    if c == 'O' {
                        println!("Set {},{}", column,row);
                        if mode == "single" {
                            r_pentomino.set_primitive((column,row), 1);
                        } else {
                            r_pentomino_parallel.set_primitive((column,row), 1);
                        }
                        
                    }

                    column += 1;
                }

                //Increment row by 1 
                row += 1;
                column = 0; //Reset column 
            }
        }
        
    }

    r_pentomino.cleanup();
    r_pentomino_parallel.cleanup();
    

    if mode == "single" {
        r_pentomino.generation = 0;
        // r_pentomino.set((0,0), 1);
        // r_pentomino.set((0,1), 1);
        // r_pentomino.set((1,1), 1);
        // r_pentomino.set((-1,0), 1);
        // r_pentomino.set((0,-1), 1);
    } else {
        r_pentomino_parallel.generation = 0;
        // r_pentomino_parallel.set((0,0), 1);
        // r_pentomino_parallel.set((0,1), 1);
        // r_pentomino_parallel.set((1,1), 1);
        // r_pentomino_parallel.set((-1,0), 1);
        // r_pentomino_parallel.set((0,-1), 1);
        name = "Conrad's Game of Life - Parallel"
    }
    

    let window_settings = WindowSettings::new(name, [WINDOW_WIDTH,WINDOW_HEIGHT]);

    let mut window: PistonWindow = window_settings.build().unwrap();


    let mut events = window.events();

    let mut running_average_time = 0.0;
    let mut running_average_count = 0.0;

    let font_path = "src/Quicksand-Regular.ttf";
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font_path, factory).unwrap();

    let mut running = true;
    
    let mut time_taken = 0.0;

    let mut mouse_pos:[f64;2] = [0.0,0.0];
    let mut mouse_middle_down = false;
    let mut mouse_last_pos:[f64;2] = [0.0,0.0];
    let mut prevoffset_x = offset_x;
    let mut prevoffset_y = offset_y;

    while let Some(e) = events.next(&mut window) {

        if let Some(Button::Keyboard(key)) = e.press_args() {
           if key == Key::Space {
             running = !running;
           }
        };

        if let Some(Button::Mouse(mouse_btn)) = e.release_args() {
            if mouse_btn == MouseButton::Middle {
                //Stop moving 
                mouse_middle_down = false;
            }
        };

        if let Some(Button::Mouse(mouse_btn)) = e.press_args() {
           let x = (((mouse_pos[0] - offset_x) - (WINDOW_WIDTH as f64/2.0)) / zoom).floor() as isize;
           let y = (((mouse_pos[1] - offset_y) - (WINDOW_HEIGHT as f64/2.0)) / zoom).floor() as isize;

           if mouse_btn == MouseButton::Middle {
              mouse_middle_down=true;
           }
           if mouse_btn == MouseButton::Left {
              // Set Alive
              if mode == "single" {
                r_pentomino.set((x,y),1); 
              } else {
                r_pentomino_parallel.set((x,y),1); 
              }
            
           }
           if mouse_btn == MouseButton::Right {
              // Set Dead
              if mode == "single" {
                r_pentomino.set((x,y),0); 
              } else {
                r_pentomino_parallel.set((x,y),0); 
              }
           }
        };
       
        if let Some(mot) = e.mouse_cursor_args(){
            mouse_pos =  mot;
            if mouse_middle_down == false {
                mouse_last_pos = mot;
                prevoffset_x = offset_x;
                prevoffset_y = offset_y;
            }
            if mouse_middle_down == true {
                offset_x = mot[0] - mouse_last_pos[0] + prevoffset_x;
                offset_y = mot[1] - mouse_last_pos[1] + prevoffset_y;
            }
        };

        if let Some(scroll) = e.mouse_scroll_args(){
            let zoom_power = 0.1;
            if scroll[1] == 1.0 {
                zoom += zoom_power;
            }
            if scroll[1] == -1.0 {
                zoom -= zoom_power;
            }
        };

        if e.render_args().is_some() {
            //Render!

            if running {
                let start_time = time::precise_time_ns() as f64;
                if mode == "single" {
                    r_pentomino.advance(); 
                } else {
                    r_pentomino_parallel.advance(); 
                }
                time_taken =  ((time::precise_time_ns() as f64 - start_time) as f64) / 1000000000.0;
                running_average_count += 1.0;
                running_average_time += time_taken;
            }
            

            //Draw the grid 
            if mode == "single" {
                r_pentomino.draw(&mut window,&e,zoom,offset_x,offset_y);
            } else {
                r_pentomino_parallel.draw(&mut window,&e,zoom,offset_x,offset_y);
            }
            
            let mut generation = r_pentomino.generation;
            if mode == "parallel" {
                generation = r_pentomino_parallel.generation;
            }

            let display_one:String = format!("Average time per generation: {0:.4} seconds.", running_average_time / running_average_count);  
            let display_two:String = format!("Time taken for last generation: {0:.4} seconds.", time_taken);  
            let display_three:String = format!("Generation: {}", generation);  

             //Render text
             window.draw_2d(&e, |c, g| {
                let x = 5.0;
                let y = 15.0;
                let line_spacing = 15.0;

                let mut transform = c.transform.trans(x, y);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 11).draw(
                    &display_one,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );

                transform = c.transform.trans(x, y+line_spacing);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 11).draw(
                    &display_two,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );

                transform = c.transform.trans(x, y+line_spacing*2.0);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 11).draw(
                    &display_three,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );


            });
        }

        if e.update_args().is_some() {
            //Update
        }
    }

}
