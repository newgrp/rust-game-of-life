use std::collections::HashSet;
use std::collections::hash_set;
use std::collections::HashMap;
use std;

use common::{LifeAlgorithm,Bounds};

pub struct Life {
    pub generation: i64,
    pub cells: HashMap<(isize, isize), bool>,
    rect: Bounds,
}

impl Life {
    pub fn new() -> Life {
        Life { generation: 0,cells: HashMap::new(), rect: Bounds::new()}
    }
    fn next_val(&self, x:isize, y:isize) -> bool {
        let mut neighbors: i8 = 0;
        for (i,j) in self.get_adjacent(x,y) {
            if self.cells.contains_key(&(i,j)) {
                if self.cells[&(i,j)] == true { neighbors+=1; }
            }
        }
        if (neighbors == 3) | ((neighbors == 2) & (self.cells[&(x,y)] == true)) {
            true
        } else {
            false
        }
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
}

impl LifeAlgorithm<hash_set::IntoIter<(isize, isize)>> for Life {
    fn advance_by(&mut self,count:u64){
        for _ in 0..count {
            let mut cells_new: HashMap<(isize, isize), bool> = HashMap::new();
            for &(x,y) in self.cells.keys() {
               cells_new.insert((x,y), self.next_val(x,y));
            }
            self.cells = cells_new;
            self.clean_up();
            self.generation += 1;
        }
        
    }

    fn set(&mut self,cell:(isize,isize), value: bool){
        let x = cell.0;
        let y = cell.1;
        if !self.cells.contains_key(&(x,y)) {
            self.cells.insert((x,y), value);
        }
        else {
            if let Some(z) = self.cells.get_mut(&(x,y)) {
                *z = value;
            }
        }
    }

    fn clean_up(&mut self){
        self.rect.x_min = std::isize::MAX;
        self.rect.x_max = std::isize::MIN;
        self.rect.y_min = std::isize::MAX;
        self.rect.y_max = std::isize::MIN;
        let mut to_add: Vec<(isize, isize)> = vec![];
        let mut to_del: Vec<(isize, isize)> = vec![];
        for (&(x,y),v) in &self.cells {
            if *v == true {
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
                        if self.cells[&(i,j)] == true {
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
            self.cells.insert((x,y), false);
            self.rect.update_bounds(x,y);
        }
        for (x,y) in to_del {
            self.cells.remove(&(x,y));
        }
    }

    fn get_generation(&self) -> i64 {
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
        self.cells.drain();
    }

    fn live_cells(&self) -> hash_set::IntoIter<(isize, isize)> {
        let mut out: HashSet<(isize, isize)> = HashSet::new();
        for (key, value) in self.cells {
            if value {
                out.insert(key);
            }
        }
        out.into_iter()
    }

}
