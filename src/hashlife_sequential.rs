#![allow(unused_variables,dead_code)]

use std::sync::Arc;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

use life_traits::{Bounds,LifeObject};

#[derive(PartialEq, Eq, Hash, Clone)]
enum LifeData {
    Leaf(bool),
    Split(Arc<LifeNode>, Arc<LifeNode>, Arc<LifeNode>, Arc<LifeNode>),
}

use self::LifeData::*;

#[derive(PartialEq, Eq, Hash, Clone)]
struct LifeNode {
    level: u64,
    info: LifeData,
}

impl LifeNode {
    /// Creates a new leaf LifeNode object with state v.
    pub fn new(v: bool) -> LifeNode {
        LifeNode { level: 0, info: Leaf(v) }
    }

    /// Creates a new non-leaf LifeNode object with the specified components. If the components'
    /// levels do not match, then the thread panics.
    pub fn with_components(ne: Arc<LifeNode>,
                           nw: Arc<LifeNode>,
                           sw: Arc<LifeNode>,
                           se: Arc<LifeNode>) -> LifeNode {
        assert_eq!(ne.get_level(), nw.get_level());
        assert_eq!(nw.get_level(), sw.get_level());
        assert_eq!(sw.get_level(), se.get_level());
        LifeNode { level: ne.get_level()+1, info: Split(ne, nw, sw, se) }
    }

    /// Turns a square grid of same-level LifeNodes into a single LifeNode
//    pub fn from_grid(grid: Vec<Vec<Arc<LifeNode>>>, hashes: HashMap<LifeNode, Arc<LifeNode>>) -> LifeNode {
//        if grid.len() == 2 {
//            LifeNode::with_components(grid[1][1],
//                                      grid[0][1],
//                                      grid[0][0],
//                                      grid[1][0])
//        } else {
//            let temp_ne = 
//        }
//    }

    /// Returns the level of the node.
    pub fn get_level(&self) -> u64 {
        self.level
    }

    /// Returns whether the current node is alive or dead as a bool. If the node is not a leaf, then
    /// the thread panics.
    pub fn is_alive(&self) -> bool {
        if let &Leaf(v) = &self.info {
            v
        } else {
            panic!("Node is not leaf.");
        }
    }

    /// Returns the northeast quadrant of the node. If the node is a leaf, then the thread panics.
    pub fn get_ne(&self) -> Arc<LifeNode> {
        if let &Split(ref ne, ref nw, ref sw, ref se) = &self.info {
            (*ne).clone()
        } else {
            panic!("Node is leaf.");
        }
    }

    /// Returns the northwest quadrant of the node. If the node is a leaf, then the thread panics.
    pub fn get_nw(&self) -> Arc<LifeNode> {
        if let &Split(ref ne, ref nw, ref sw, ref se) = &self.info {
            (*nw).clone()
        } else {
            panic!("Node is leaf.");
        }
    }

    /// Returns the southwest quadrant of the node. If the node is a leaf, then the thread panics.
    pub fn get_sw(&self) -> Arc<LifeNode> {
        if let &Split(ref ne, ref nw, ref sw, ref se) = &self.info {
            (*sw).clone()
        } else {
            panic!("Node is leaf.");
        }
    }

    /// Returns the southeast quadrant of the node. If the node is a leaf, then the thread panics.
    pub fn get_se(&self) -> Arc<LifeNode> {
        if let &Split(ref ne, ref nw, ref sw, ref se) = &self.info {
            (*se).clone()
        } else {
            panic!("Node is leaf.");
        }
    }

    /// Returns half the side length of the square represented by this node.
    pub fn side_len(&self) -> isize {
        (2 as isize) << ((self.level-1) as isize)
    }

    /// Returns true if the (x,y) coordinates lie inside the current node and false otherwise.
    pub fn is_inside(&self, x: isize, y: isize) -> bool {
        let bound = self.side_len();
        (x == 0 && y == 0) || (x >= -bound && x < bound && y >= -bound && y < bound)
    }

    /// Returns the node of the specified level at the specified offset of nodes of that size from
    /// the center. The southwest-most subnode of ne of the specified level has offset (0,0), the
    /// one to its right has offset (1,0), etc. If this node is smaller than the one being searched
    /// for, the thread panics.
    pub fn get_chunk(&self, lvl: u64, x: isize, y: isize,
                     hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert!(self.level >= lvl);
        let cap = (2 as isize) << ((self.level-lvl) as isize);
        assert!((x == 0 && y == 0) || (x >= -cap && x < cap && y >= -cap && y < cap));
        if self.level == lvl {
            self.clone().do_arc(hashes)
        } else if self.level == lvl+1 {
            match (x,y) {
                (0 , 0 ) => self.get_ne(),
                (-1, 0 ) => self.get_nw(),
                (-1, -1) => self.get_sw(),
                (0 , -1) => self.get_se(),
                _        => panic!("This is really weird.")
            }
        } else if x >= 0 && y >= 0 {
            self.get_ne().get_chunk(lvl, x-cap/2, y-cap/2, hashes)
        } else if x <  0 && y >= 0 {
            self.get_nw().get_chunk(lvl, x+cap  , y-cap/2, hashes)
        } else if x <  0 && y <  0 {
            self.get_sw().get_chunk(lvl, x+cap  , y+cap  , hashes)
        } else {
            self.get_se().get_chunk(lvl, x-cap/2, y+cap  , hashes)
        }
    }

    /// Returns the alive/dead value of the node at the given coordinates, assuming that the
    /// southwest corner of the northeast subnode is (0,0). If the coordinates are invalid, then
    /// the thread panics.
    pub fn get_value(&self, x: isize, y: isize) -> bool {
        assert!(self.is_inside(x,y));
        let bound = self.side_len();
        if self.level == 0 {
            self.is_alive()
        } else if x >= 0 && y >= 0 {
            self.get_ne().get_value(x+bound/2, y+bound/2)
        } else if x < 0 && y >= 0 {
            self.get_nw().get_value(x+bound  , y+bound/2)
        } else if x < 0 && y < 0 {
            self.get_sw().get_value(x+bound  , y+bound  )
        } else {
            self.get_se().get_value(x+bound/2, y+bound  )
        }
    }

    /// Returns the Arc corresponding to the given node in hashes or inserts it if it does not
    /// already exist.
    fn do_arc(self, hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        let or_value = Arc::new(self.clone());
        hashes.entry(self).or_insert(or_value).clone()
    }

    pub fn change_value(&self, x: isize, y: isize, val: bool,
                        hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert!(self.is_inside(x,y));
        let bound = self.side_len();
        if self.level == 0 {
            LifeNode::new(val).do_arc(hashes)
        } else if x >= 0 && y >= 0 {
            LifeNode::with_components(self.get_ne().change_value(x+bound/2, y+bound/2, val, hashes),
                                      self.get_nw(),
                                      self.get_sw(),
                                      self.get_se()).do_arc(hashes)
        } else if x < 0 && y >= 0 {
            LifeNode::with_components(self.get_ne(),
                                      self.get_nw().change_value(x+bound  , y+bound/2, val, hashes),
                                      self.get_sw(),
                                      self.get_se()).do_arc(hashes)
        } else if x < 0 && y < 0 {
            LifeNode::with_components(self.get_ne(),
                                      self.get_nw(),
                                      self.get_sw().change_value(x+bound  , y+bound  , val, hashes),
                                      self.get_se()).do_arc(hashes)
        } else {
            LifeNode::with_components(self.get_ne(),
                                      self.get_nw(),
                                      self.get_sw(),
                                      self.get_se().change_value(x+bound/2, y+bound  , val, hashes)).do_arc(hashes)
        }
    }

    /// Returns true if all the border nodes of self with level self.level-2 are equal to target
    /// and false otherwise. If self.level < 2, the thread panics.
    pub fn is_uniform_border(&self,
                             hashes: &HashMap<LifeNode, Arc<LifeNode>>,
                             target: Arc<LifeNode>) -> bool {
        assert!(self.level >= 2);
        let mut acc = true;
        for node in vec![self.get_ne().get_se(),
                         self.get_ne().get_ne(),
                         self.get_ne().get_nw(),
                         self.get_nw().get_ne(),
                         self.get_nw().get_nw(),
                         self.get_nw().get_sw(),
                         self.get_sw().get_nw(),
                         self.get_sw().get_sw(),
                         self.get_sw().get_se(),
                         self.get_se().get_sw(),
                         self.get_se().get_se(),
                         self.get_se().get_ne()] {
            acc &= node == target;
            if !acc {
                break;
            }
        }
        acc
    }

    /// Returns the node representing the centered square inside the current node of half the side
    /// length. If level < 2, then the thread panics.
    pub fn centered_forward(&self,
                            hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert!(self.level >= 2);
        LifeNode::with_components(self.get_ne().get_sw(),
                                  self.get_nw().get_se(),
                                  self.get_sw().get_ne(),
                                  self.get_se().get_nw()).do_arc(hashes)
    }

    /// Returns the node representing the east half of w and the west half of e, as if they were
    /// horizontally adjacent and lined up, with w on the west side and e on the east. If the
    /// levels of w and e do not match or they are not at least level 1, then the thread panics.
    pub fn horizontal_forward(w: Arc<LifeNode>,
                              e: Arc<LifeNode>,
                              hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert_eq!(w.get_level(), e.get_level());
        assert!(w.get_level() >= 1);
        LifeNode::with_components(e.get_nw(),
                                  w.get_ne(),
                                  w.get_se(),
                                  e.get_sw()).do_arc(hashes)
    }

    /// Returns the node representing the south half of n and the north half of s, as if they were
    /// vertically adjacent and lined up, with n on the north side and s on the south. If the
    /// levels of n and s do not match or they are not at least level 1, then the thread panics.
    pub fn vertical_forward(n: Arc<LifeNode>,
                            s: Arc<LifeNode>,
                            hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert_eq!(n.get_level(), s.get_level());
        assert!(n.get_level() >= 1);
        LifeNode::with_components(n.get_se(),
                                  n.get_sw(),
                                  s.get_nw(),
                                  s.get_ne()).do_arc(hashes)
    }

    /// Returns the next value of a cell, given its neighbors. If the neighbors object does not have
    /// length 8, the thread panics.
    fn next_value_from_neighbors(current: bool,
                                 neighbors: Vec<bool>,
                                 hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert_eq!(neighbors.len(), 8);
        let mut neighbors_sum: u8 = 0;
        for n in neighbors {
            neighbors_sum += if n { 1 } else { 0 };
        }
        LifeNode::new((neighbors_sum == 3) || (current && (neighbors_sum == 2))).do_arc(hashes)
    }

    /// Returns the node representing the centered square inside the current node of half the side
    /// length, advanced by 2^(level-2) generations. If level < 2, then the thread panics.
    pub fn advanced_center(&self,
                           hashes: &mut HashMap<LifeNode, Arc<LifeNode>>,
                           memos: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        if let Some(out) = memos.get(self) {
            return out.clone()
        }
        assert!(self.level >= 2);
        if self.level == 2 {
            let new_ne = LifeNode::next_value_from_neighbors(self.get_ne().get_sw().is_alive(),
                                                             vec![self.get_ne().get_se().is_alive(),
                                                                  self.get_ne().get_ne().is_alive(),
                                                                  self.get_ne().get_nw().is_alive(),
                                                                  self.get_nw().get_ne().is_alive(),
                                                                  self.get_nw().get_se().is_alive(),
                                                                  self.get_sw().get_ne().is_alive(),
                                                                  self.get_se().get_nw().is_alive(),
                                                                  self.get_se().get_ne().is_alive()],
                                                             hashes);
            let new_nw = LifeNode::next_value_from_neighbors(self.get_nw().get_se().is_alive(),
                                                             vec![self.get_ne().get_sw().is_alive(),
                                                                  self.get_ne().get_nw().is_alive(),
                                                                  self.get_nw().get_ne().is_alive(),
                                                                  self.get_nw().get_nw().is_alive(),
                                                                  self.get_nw().get_sw().is_alive(),
                                                                  self.get_sw().get_nw().is_alive(),
                                                                  self.get_sw().get_ne().is_alive(),
                                                                  self.get_se().get_nw().is_alive()],
                                                             hashes);
            let new_sw = LifeNode::next_value_from_neighbors(self.get_sw().get_ne().is_alive(),
                                                             vec![self.get_se().get_nw().is_alive(),
                                                                  self.get_ne().get_sw().is_alive(),
                                                                  self.get_nw().get_se().is_alive(),
                                                                  self.get_nw().get_sw().is_alive(),
                                                                  self.get_sw().get_nw().is_alive(),
                                                                  self.get_sw().get_sw().is_alive(),
                                                                  self.get_sw().get_se().is_alive(),
                                                                  self.get_se().get_sw().is_alive()],
                                                             hashes);
            let new_se = LifeNode::next_value_from_neighbors(self.get_se().get_nw().is_alive(),
                                                             vec![self.get_se().get_ne().is_alive(),
                                                                  self.get_ne().get_se().is_alive(),
                                                                  self.get_ne().get_sw().is_alive(),
                                                                  self.get_nw().get_se().is_alive(),
                                                                  self.get_sw().get_ne().is_alive(),
                                                                  self.get_sw().get_se().is_alive(),
                                                                  self.get_se().get_sw().is_alive(),
                                                                  self.get_se().get_se().is_alive()],
                                                             hashes);
            return LifeNode::with_components(new_ne, new_nw, new_sw, new_se).do_arc(hashes)
        } else {
            let node_ce = LifeNode::vertical_forward(self.get_ne(), self.get_se(), hashes).advanced_center(hashes, memos);
            let node_ne = self.get_ne().advanced_center(hashes, memos);
            let node_nc = LifeNode::horizontal_forward(self.get_nw(), self.get_ne(), hashes).advanced_center(hashes, memos);
            let node_nw = self.get_nw().advanced_center(hashes, memos);
            let node_cw = LifeNode::vertical_forward(self.get_nw(), self.get_sw(), hashes).advanced_center(hashes, memos);
            let node_sw = self.get_sw().advanced_center(hashes, memos);;
            let node_sc = LifeNode::horizontal_forward(self.get_sw(), self.get_se(), hashes).advanced_center(hashes, memos);
            let node_se = self.get_se().advanced_center(hashes, memos);
            let node_cc = self.centered_forward(hashes).advanced_center(hashes, memos);
            let new_ne = LifeNode::with_components(node_ne.clone(),
                                                   node_nc.clone(),
                                                   node_cc.clone(),
                                                   node_ce.clone()).advanced_center(hashes, memos);
            let new_nw = LifeNode::with_components(node_nc.clone(),
                                                   node_nw.clone(),
                                                   node_cw.clone(),
                                                   node_cc.clone()).advanced_center(hashes, memos);
            let new_sw = LifeNode::with_components(node_cc.clone(),
                                                   node_cw.clone(),
                                                   node_sw.clone(),
                                                   node_sc.clone()).advanced_center(hashes, memos);
            let new_se = LifeNode::with_components(node_ce.clone(),
                                                   node_cc.clone(),
                                                   node_sc.clone(),
                                                   node_se.clone()).advanced_center(hashes, memos);
            LifeNode::with_components(new_ne, new_nw, new_sw, new_se).do_arc(hashes)
        }
    }
}

struct Life {
    generation: u64,
    hashes: HashMap<LifeNode, Arc<LifeNode>>,
    advanced_centers: HashMap<LifeNode, Arc<LifeNode>>,
    dead_squares: Vec<Arc<LifeNode>>,
    root: Arc<LifeNode>,
}

impl Life {
    /// Returns a new completely dead board. The root node will be level 3.
    pub fn new() -> Life {
        let mut hashes_temp: HashMap<LifeNode, Arc<LifeNode>> = HashMap::new();
        let dead_cell = LifeNode::new(false).do_arc(&mut hashes_temp);
        let root_temp = LifeNode::with_components(dead_cell.clone(),
                                                  dead_cell.clone(),
                                                  dead_cell.clone(),
                                                  dead_cell.clone()).do_arc(&mut hashes_temp);
        let mut out = Life { generation: 0,
                             hashes: hashes_temp.clone(),
                             advanced_centers: HashMap::new(),
                             dead_squares: vec![dead_cell, root_temp.clone()],
                             root: root_temp };
        out.pad().pad();
        out
    }

    /// Returns the canonical completely dead node of the given level.
    fn canonical_dead(lvl: usize,
                      dead_squares: &mut Vec<Arc<LifeNode>>,
                      hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        if dead_squares.len() > lvl {
            dead_squares[lvl].clone()
        } else {
            let out = LifeNode::with_components(Life::canonical_dead(lvl-1, dead_squares, hashes),
                                                Life::canonical_dead(lvl-1, dead_squares, hashes),
                                                Life::canonical_dead(lvl-1, dead_squares, hashes),
                                                Life::canonical_dead(lvl-1, dead_squares, hashes)).do_arc(hashes);
            dead_squares.push(out.clone());
            out
        }
    }

    /// Resizes the root node so it has twice the side length but all border squares of level
    /// root.level-2 are empty and the inner square of level root.level-1 is identical to the
    /// original root. Returns self.
    fn pad(&mut self) -> &mut Life {
        let lvl = self.root.get_level();
        let padder = Life::canonical_dead((lvl-1) as usize, &mut self.dead_squares, &mut self.hashes);
        let new_ne = LifeNode::with_components(padder.clone(),
                                               padder.clone(),
                                               self.root.get_ne(),
                                               padder.clone()).do_arc(&mut self.hashes);
        let new_nw = LifeNode::with_components(padder.clone(),
                                               padder.clone(),
                                               padder.clone(),
                                               self.root.get_nw()).do_arc(&mut self.hashes);
        let new_sw = LifeNode::with_components(self.root.get_sw(),
                                               padder.clone(),
                                               padder.clone(),
                                               padder.clone()).do_arc(&mut self.hashes);
        let new_se = LifeNode::with_components(padder.clone(),
                                               self.root.get_sw(),
                                               padder.clone(),
                                               padder.clone()).do_arc(&mut self.hashes);
        self.root = LifeNode::with_components(new_ne,
                                              new_nw,
                                              new_sw,
                                              new_se).do_arc(&mut self.hashes);
        self
    }

    /// Resizes the root node so that all live cells are contained in the center inner square of
    /// level root.level-2. Returns self.
    fn expand_to_fit(&mut self) -> &mut Life {
        let dead_large = Life::canonical_dead((self.root.get_level()-2) as usize,
                                              &mut self.dead_squares,
                                              &mut self.hashes);
        let dead_small = Life::canonical_dead((self.root.get_level()-3) as usize,
                                              &mut self.dead_squares,
                                              &mut self.hashes);
        if !self.root.is_uniform_border(&self.hashes, dead_large) {
            self.pad().pad();
            self
        } else if !LifeNode::with_components(self.root.get_ne().get_sw(),
                                             self.root.get_nw().get_se(),
                                             self.root.get_sw().get_ne(),
                                             self.root.get_se().get_nw()).is_uniform_border(&self.hashes,
                                                                                            dead_small) {
            self.pad();
            self
        } else {
            self
        }
    }

    /// Advances the board by 2^(self.root.level-2) generations.
    pub fn advance_arbitrary(&mut self) -> &mut Life {
        self.generation += (2 as u64) << ((self.root.get_level()-2) as u64);
        self.root = self.root.advanced_center(&mut self.hashes, &mut self.advanced_centers);
        self.expand_to_fit()
    }
}

impl LifeObject for Life {
    /// Creates a new Life from a .cells file.
    fn from_file(filename: &str) -> Life {
        assert_eq!(&filename[filename.len()-6..filename.len()], ".cells");
        if let Ok(f) = File::open(filename) {
            let f = BufReader::new(f);
            let mut out = Life::new();
            let mut j = 0;
            for (lno, line) in f.lines().enumerate() {
                let line = line.unwrap();
                if &line[0..0] != "!" {
                    for (i,c) in line.chars().enumerate() {
                        match c {
                            '.' => (),
                            'O' => out.set(i as isize, -j, true),
                            _   => panic!("Invalid character {} at position {}, {} in file {}",
                                          c, lno, i, filename),
                        };
                    }
                    j += 1;
                }
            }
            out
        } else {
            panic!("Could not open file {}", filename);
        }
    }

    /// Returns the current generation.
    fn get_generation(&self) -> u64 {
        self.generation
    }

    /// Returns a bounds object containing all live cells.
    fn get_bounds(&self) -> Bounds {
        Bounds::from_half_side(self.root.side_len()/2)
    }

    /// Returns the alive/dead value at the given coordinates. The southwest corner of the
    /// northeast quadrant is assumed to be (0,0).
    fn get_value(&self, x: isize, y: isize) -> bool {
        let bound: isize = (2 as isize) << ((self.root.get_level()-1) as isize);
        if x < -bound || y < -bound || x >= bound || y >= bound {
            false
        } else {
            self.root.get_value(x,y)
        }
    }

    /// Sets the specified cell to the specified value.
    fn set(&mut self, x: isize, y: isize, val: bool) {
        let mut bound = self.root.side_len();
        while x < -bound || y < -bound || x >= bound || y >= bound {
            self.pad();
            bound = self.root.side_len();
        }
        self.root = self.root.change_value(x, y, val, &mut self.hashes);
    }

    /// Toggles the value of the specified cell.
    fn toggle(&mut self, x: isize, y: isize) {
        let new_val = !self.get_value(x, y);
        self.set(x, y, new_val);
    }

//    /// Advances the game by the specified number of generations.
//    fn advance_by(&mut self, time: u64) -> &mut Life {
//        let mut time = time;
//        while time > 0 {
//            let amt = self.root.side_len()/2; // the starting side length of the live region of root
//            if time >= amt {
//                self.generation += amt;
//                self.root = self.root.advanced_center(&mut self.hashes, &mut self.advanced_centers);
//                self.expand_to_fit();
//                time -= amt
//            } else {
//                let ex = 63u64 - time.leading_zeros();
//                let bin = 2u64 << ex; // number of generations we advance by in this iteration of the loop
//                let side = 2*bin; // the side length of the squares we will recompose root from
//                let num = amt/side+1; // the number of squares on each side needed to tile root in these
//                let values: Vec<Arc<LifeNode>> = vec![];
//                for i in -num/2..num/2 {
//                    values.push(vec![]);
//                    for j in -num/2..num/2 {
//                        values[i+num/2].push(LifeNode::with_components(self.root.get_chunk(ex+1, i  , j  , self.hashes),
//                                                                       self.root.get_chunk(ex+1, i-1, j  , self.hashes),
//                                                                       self.root.get_chunk(ex+1, i-1, j-1, self.hashes),
//                                                                       self.root.get_chunk(ex+1, i  , j-1, self.hashes)).advanced_center(self.hashes, self.advanced_centers));
//                    }
//                }
//                self.generation += bin;
//                self.expand_to_fit();
//                time -= bin;
//            }
//        }
//        self
//    }
}
