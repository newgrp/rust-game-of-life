extern crate num_cpus;
extern crate rand;


use std::cmp::{min,max};
use std::sync::Arc;
use std::collections::HashSet;
use std::collections::hash_set;
use std::collections::HashMap;
use std::thread;
use std;

use common::{LifeAlgorithm,Bounds};

// Extra functionality for bounds 
impl Bounds {
    fn merge(&mut self, other:Bounds) {
        self.x_min = min(self.x_min, other.x_min);
        self.x_max = max(self.x_max, other.x_max);
        self.y_min = min(self.y_min, other.y_min);
        self.y_max = max(self.y_max, other.y_max);
    }
}

#[derive(Clone)]
pub struct Life {
    pub generation: u64,
    pub cells: Arc<HashMap<(isize, isize), bool>>,
    parts: Vec<Arc<HashSet<(isize, isize)>>>,
    rect: Bounds,
    num_threads:usize,
}

impl Life {
    pub fn new() -> Life {
        let num_threads = num_cpus::get() * 2; //Use twice as many threads as we have cores
        Life { generation: 0, cells: Arc::new(HashMap::new()), parts: vec![Arc::new(HashSet::new()); num_threads], rect: Bounds::new(), num_threads:num_threads }
    }

    fn cells_access_record(s:&str) {
        println!("Arc::get_mut(&mut self.cells) returned None at {}", s);
    }

    fn next_val_from_arc(cells_ref:&Arc<HashMap<(isize, isize), bool>>, x:isize, y:isize) -> bool {
        let mut neighbors: i8 = 0;
        for (i,j) in Life::get_adjacent(x,y) {
            if cells_ref.contains_key(&(i,j)) {
                if cells_ref[&(i,j)] == true { neighbors+=1; }
            }
        }
        if (neighbors == 3) | ((neighbors == 2) & (cells_ref[&(x,y)] == true)) {
            true
        } else {
            false
        }
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
}

impl LifeAlgorithm<hash_set::IntoIter<(isize, isize)>> for Life {
    fn advance_by(&mut self, count:u64){
        for _ in 0..count {
            let mut thread_handles = vec![];
            for k in 0..self.num_threads {
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
            let mut cells_new: Vec<HashMap<(isize, isize), bool>> = vec![];
            for hand in thread_handles {
                cells_new.push(hand.join().unwrap());
            }
            for k in 0..self.num_threads {
                for (&(x,y),v) in &cells_new[k] {
                    if let Some(re) = Arc::get_mut(&mut self.cells) {
                        if let Some(z) = (*re).get_mut(&(x,y)) {
                            *z = *v;
                        }
                    } else {
                        Life::cells_access_record("Life::advance");
                    }
                }
            }
            self.clean_up();
            self.generation += 1;
        }
        
    }

    fn set(&mut self, (x, y): (isize, isize), v: bool){
        if !self.cells.contains_key(&(x,y)) {
            if let Some(re) = Arc::get_mut(&mut self.cells) {
                (*re).insert((x,y), v);
            } else {
                Life::cells_access_record("Life::set, does not contain key");
            }
            let ind = rand::random::<usize>()%self.num_threads;
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
                Life::cells_access_record("Life::set, contains key");
            }
        }
    }

    fn clean_up(&mut self){
        self.rect.x_min = std::isize::MAX;
        self.rect.x_max = std::isize::MIN;
        self.rect.y_min = std::isize::MAX;
        self.rect.y_max = std::isize::MIN;
        let mut thread_handles = vec![];
        for k in 0..self.num_threads {
            let my_cells = self.cells.clone();
            let my_part = self.parts[k].clone();
            let mut temp = self.rect.clone();
            thread_handles.push(thread::spawn(move || {
                let mut to_add: Vec<(isize, isize)> = vec![];
                let mut to_del: Vec<(isize, isize)> = vec![];
                for &(x,y) in my_part.iter() {
                    if my_cells[&(x,y)] == true {
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
                                if my_cells[&(i,j)] == true {
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
        for k in 0..self.num_threads {
            for &(x,y) in &to_adds[k] {
                if !self.cells.contains_key(&(x,y)) {
                    if let Some(re) = Arc::get_mut(&mut self.cells) {
                        (*re).insert((x,y), false);
                    } else {
                        Life::cells_access_record("Life::cleanup, inserting new cells");
                    }
                    self.rect.update_bounds(x,y);
                    let ind = rand::random::<usize>()%self.num_threads;
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
                    Life::cells_access_record("Life::cleanup, removing cells");
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

    fn get_generation(&self) -> u64 {
        self.generation
    }

    fn get_bounds(&self) -> Bounds {
        self.rect.clone()
    }

    fn get_value(&self, cell: (isize, isize)) -> bool {
        if let Some(v) = self.cells.get(&cell) {
            *v
        } else {
            false
        }
    }

    fn clear(&mut self) {
        if let Some(re) = Arc::get_mut(&mut self.cells) {
            (*re).drain();
        } else {
            Life::cells_access_record("Life::clear");
        }
    }

    fn live_cells(&self) -> hash_set::IntoIter<(isize, isize)> {
        let mut out: HashSet<(isize, isize)> = HashSet::new();
        for (key, value) in self.cells.iter() {
            if *value {
                out.insert(*key);
            }
        }
        out.into_iter()
    }
}
