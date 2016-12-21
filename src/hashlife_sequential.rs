use std::sync::Arc;
use std::collections::HashMap;

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

    /// Returns the Arc corresponding to the given node in hashes or inserts it if it does not
    /// already exist.
    pub fn do_arc(self, hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        let or_value = Arc::new(self.clone());
        hashes.entry(self).or_insert(or_value).clone()
    }

    /// Returns the node representing the centered square inside the current node of half the side
    /// length. If level < 2, then the thread panics.
    pub fn centered_forward(&self,
                            hashes: &mut HashMap<LifeNode, Arc<LifeNode>>,
                            memos: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert!(self.level >= 2);
        LifeNode::with_components(self.get_ne().get_sw(),
                                  self.get_nw().get_se(),
                                  self.get_sw().get_ne(),
                                  self.get_se().get_nw()).advanced_center(hashes, memos)
    }

    /// Returns the node representing the east half of w and the west half of e, as if they were
    /// horizontally adjacent and lined up, with w on the west side and e on the east. If the
    /// levels of w and e do not match or they are not at least level 1, then the thread panics.
    pub fn horizontal_forward(w: Arc<LifeNode>,
                              e: Arc<LifeNode>,
                              hashes: &mut HashMap<LifeNode, Arc<LifeNode>>,
                              memos: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert_eq!(w.get_level(), e.get_level());
        assert!(w.get_level() >= 1);
        LifeNode::with_components(e.get_nw(),
                                  w.get_ne(),
                                  w.get_se(),
                                  e.get_sw()).advanced_center(hashes, memos)
    }

    /// Returns the node representing the south half of n and the north half of s, as if they were
    /// vertically adjacent and lined up, with n on the north side and s on the south. If the
    /// levels of n and s do not match or they are not at least level 1, then the thread panics.
    pub fn vertical_forward(n: Arc<LifeNode>,
                            s: Arc<LifeNode>,
                            hashes: &mut HashMap<LifeNode, Arc<LifeNode>>,
                            memos: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert_eq!(n.get_level(), s.get_level());
        assert!(n.get_level() >= 1);
        LifeNode::with_components(n.get_se(),
                                  n.get_sw(),
                                  s.get_nw(),
                                  s.get_ne()).advanced_center(hashes, memos)
    }

    /// Returns the next value of a cell, given its neighbors. If the neighbors object does not have
    /// length 8, the thread panics.
    fn next_value_from_neighbors(current: bool, neighbors: Vector<bool>, hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
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
            out.clone()
        } else {
            assert!(self.level >= 2);
            use self::LifeNode::next_value_from_neighbors as next;
            if self.level == 2 {
                let new_ne = next(self.get_ne().get_sw().is_alive(),
                                  vec![self.get_ne().get_se().is_alive(),
                                       self.get_ne().get_ne().is_alive(),
                                       self.get_ne().get_nw().is_alive(),
                                       self.get_nw().get_ne().is_alive(),
                                       self.get_nw().get_se().is_alive(),
                                       self.get_sw().get_ne().is_alive(),
                                       self.get_se().get_nw().is_alive(),
                                       self.get_se().get_ne().is_alive()],
                                  hashes);
                let new_nw = next(self.get_nw().get_se().is_alive(),
                                  vec![self.get_ne().get_sw().is_alive(),
                                       self.get_ne().get_nw().is_alive(),
                                       self.get_nw().get_ne().is_alive(),
                                       self.get_nw().get_nw().is_alive(),
                                       self.get_nw().get_sw().is_alive(),
                                       self.get_sw().get_nw().is_alive(),
                                       self.get_sw().get_ne().is_alive(),
                                       self.get_se().get_nw().is_alive()],
                                  hashes);
                let new_sw = next(self.get_sw().get_ne().is_alive(),
                                  vec![self.get_se().get_nw().is_alive(),
                                       self.get_ne().get_sw().is_alive(),
                                       self.get_nw().get_se().is_alive(),
                                       self.get_nw().get_sw().is_alive(),
                                       self.get_sw().get_nw().is_alive(),
                                       self.get_sw().get_sw().is_alive(),
                                       self.get_sw().get_se().is_alive(),
                                       self.get_se().get_sw().is_alive()],
                                  hashes);
                let new_se = next(self.get_se().get_nw().is_alive(),
                                  vec![self.get_se().get_ne().is_alive(),
                                       self.get_ne().get_se().is_alive(),
                                       self.get_ne().get_sw().is_alive(),
                                       self.get_nw().get_se().is_alive(),
                                       self.get_sw().get_ne().is_alive(),
                                       self.get_sw().get_se().is_alive(),
                                       self.get_se().get_sw().is_alive(),
                                       self.get_se().get_se().is_alive()],
                                  hashes);
                LifeNode::with_components(new_ne, new_nw, new_sw, new_se).do_arc(hashes)
            } else {
                let node_ce = LifeNode::vertical_forward(self.get_ne(), self.get_se(), hashes);
                let node_ne = self.get_ne();
                let node_nc = LifeNode::horizontal_forward(self.get_nw(), self.get_ne(), hashes);
                let node_nw = self.get_nw();
                let node_cw = LifeNode::vertical_forward(self.get_nw(), self.get_sw(), hashes);
                let node_sw = self.get_sw();
                let node_sc = LifeNode::horizontal_forward(self.get_sw(), self.get_se(), hashes);
                let node_se = self.get_se();
                let node_cc = self.centered_forward(hashes);
                let new_ne = LifeNode::with_components(node_ne, node_nc, node_cc, node_ce).advanced_center(hashes, memos);
                let new_nw = LifeNode::with_components(node_nc, node_nw, node_cw, node_cc).advanced_center(hashes, memos);
                let new_sw = LifeNode::with_components(node_cc, node_cw, node_sw, node_sc).advanced_center(hashes, memos);
                let new_se = LifeNode::with_components(node_ce, node_cc, node_sc, node_se).advanced_center(hashes, memos);
                LifeNode::with_components(new_ne, new_nw, new_sw, new_se).do_arc(hashes)
            }
        }
    }
}

struct Life {
    hashes: HashMap<LifeNode, Arc<LifeNode>>,
    advanced_centers: HashMap<LifeNode, Arc<LifeNode>>,
    root: Arc<LifeNode>,
}
