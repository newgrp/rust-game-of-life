extern crate piston_window;
extern crate time;
extern crate find_folder;

use std::iter::Iterator;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use piston_window::*;
use find_folder::Search;
use std::path::PathBuf;

mod common;
mod life_algorithms;
mod gui;

use common::LifeAlgorithm;
use gui::GUI;

fn run_life_with_gui<I: Iterator<Item=(isize, isize)>, L: LifeAlgorithm<I>>(mut life_obj: L, window: &mut PistonWindow, seed_file: PathBuf, font_file: PathBuf) {
    // Read pattern from file      
    read_seed_from_file(&mut life_obj,seed_file);

    // Get window events 
    let mut events = window.events();

    // Create Glyphs out of font 
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font_file, factory).unwrap();

    // Initialize GUI 
    let mut gui_obj = GUI::new();

    // Some variables for benchmarking
    let mut time_taken = 0.0;
    let mut running_average_count = 0.0;
    let mut running_average_time = 0.0;

    let mut do_update = true;

    // Capture all the events we need and call the gui functions for each
    while let Some(e) = events.next(window) {

        if let Some(Button::Keyboard(key)) = e.press_args() {
            gui_obj.key_press(key);
        };

        if let Some(Button::Mouse(mouse_btn)) = e.release_args() {
            gui_obj.mouse_release(mouse_btn);
        };

        if let Some(Button::Mouse(mouse_btn)) = e.press_args() {
           gui_obj.mouse_press(mouse_btn,&mut life_obj,window);
        };
       
        if let Some(mot) = e.mouse_cursor_args(){
            gui_obj.mouse_move(mot);
        };

        if let Some(scroll) = e.mouse_scroll_args(){
            gui_obj.mouse_scroll(scroll);
        };

        if let Some(_) = e.render_args() {

            do_update = true;

            //Draw the grid 
            gui_obj.draw(&life_obj,window,&e);
            
            let generation = life_obj.get_generation();
            
            let average_time_text:String = format!("Average time per generation: {0:.4} seconds.", running_average_time / running_average_count);  
            let last_time_text:String = format!("Time taken for last generation: {0:.4} seconds.", time_taken);  
            let current_generation_text:String = format!("Generation: {}", generation);  

            // Render text
             window.draw_2d(&e, |c, g| {
                let x = 5.0;
                let y = 15.0;
                let line_spacing = 15.0;

                let mut transform = c.transform.trans(x, y);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 11).draw(
                    &average_time_text,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );

                transform = c.transform.trans(x, y+line_spacing);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 11).draw(
                    &last_time_text,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );

                transform = c.transform.trans(x, y+line_spacing*2.0);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 11).draw(
                    &current_generation_text,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );


            });
        }

        if let Some(_) = e.update_args(){
            //Update

            if !gui_obj.is_paused() && do_update {
                // Record time it takes to calculate this generation step
                let start_time = time::precise_time_ns() as f64;
                // Advance the simulation 
                life_obj.advance_by(1);
                // Now calculate the time and average 
                time_taken =  ((time::precise_time_ns() as f64 - start_time) as f64) / 1000000000.0;
                running_average_count += 1.0;
                running_average_time += time_taken;
                do_update = false;
            }

        }
    }
}

fn main() {
    // Collect any command line arguments 
    // All args are optional. 
    // First is which built in pattern to start with. Defaults to r_pentomino
    // Second is which algorithm to use. Defaults to sequential. 
    let args: Vec<String> = env::args().collect();
    
    let mut seed_pattern = "r_pentomino".to_string();
    let mut mode = "sequential".to_string();

    if args.len() > 1 { seed_pattern = args[1].parse().unwrap(); }
    if args.len() > 2 { mode = args[2].parse().unwrap(); }

    // Set up Piston window 
    let mut window:PistonWindow = WindowSettings::new("Rusty Game of Life - ".to_string() + &mode, [600,400]).build().unwrap();

    // Get asset path 
    let asset_path = Search::Parents(3).for_folder("assets").unwrap();

    // Get the seed file 
    let mut init_file:PathBuf = asset_path.clone(); 
        init_file.push("game_seeds");
        init_file.push(seed_pattern + &".cells".to_string());

    // Get the font file 
    let mut font_path = asset_path.clone();
        font_path.push("fonts");
        font_path.push("Quicksand-Regular.ttf");

    // Instantiate the right algorithm based on the given mode 
    match mode.as_ref() {
        "sequential" => {let life_logic = life_algorithms::sequential::Life::new();
                         run_life_with_gui(life_logic, &mut window, init_file, font_path);
                        },
        "parallel" =>   {let life_logic = life_algorithms::parallel::Life::new();
                         run_life_with_gui(life_logic, &mut window, init_file, font_path);
                        },
        "hashlife" =>   {let life_logic = life_algorithms::hashlife::Life::new();
                         run_life_with_gui(life_logic, &mut window, init_file, font_path);
                        },
        _ => panic!("{:?} is not a recognized algorithm. See src/life_algorithms for a list of implemented algorithms.", mode),
    }


}

fn read_seed_from_file<I: Iterator<Item=(isize, isize)>, L: LifeAlgorithm<I>>(life_obj: &mut L, path: PathBuf){
    // Takes a ref to a game-of-life object and an absolute filepath, and reads the pattern
    // Expects to find a .cells file 
    assert_eq!(path.extension().unwrap(), "cells");
    // Attempt to open the file
    if let Ok(f) = File::open(&path) {
        let f = BufReader::new(f);
        let mut j = 0;

        // Clear life object first 
        (*life_obj).clear();

        // Iterate over all the lines 
        for (lno, line) in f.lines().enumerate() {
            let line = line.unwrap();
            if line != "" && &line[0..1] != "!" { // Ignore commented out lines 
                for (i,c) in line.chars().enumerate() {
                    match c {
                        '.' => (),
                        'O' => (*life_obj).set((i as isize,j), true), 
                        _   => panic!("Invalid character {} at position {}, {} in file {}",
                                      c, lno, i, path.display()),
                    };
                }
                j += 1;
            }
            // Empty lines represent empty rows 
            if line  == "" { j+= 1; }
        }
    } else {
        panic!("Could not open {}", path.display());
    }

    // Run any necessary clean up (such as shrinking the hashmap)
    (*life_obj).clean_up();
}
