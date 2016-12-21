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

    /// Returns the node representing the centered square inside the current node of half the side
    /// length. If level < 2, then the thread panics.
    pub fn centered_forward(&self, hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert!(self.level >= 2);
        let key = LifeNode::with_components(self.get_ne().get_sw(),
                                            self.get_nw().get_se(),
                                            self.get_sw().get_ne(),
                                            self.get_se().get_nw());
        let or_value = Arc::new(key.clone());
        hashes.entry(key).or_insert(or_value).clone()
    }

    /// Returns the node representing the east half of w and the west half of e, as if they were
    /// horizontally adjacent and lined up, with w on the west side and e on the east. If the
    /// levels of w and e do not match or they are not at least level 1, then the thread panics.
    pub fn horizontal_forward(w: Arc<LifeNode>, e: Arc<LifeNode>, hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert_eq!(w.get_level(), e.get_level());
        assert!(w.get_level() >= 1);
        let key = LifeNode::with_components(e.get_nw(),
                                            w.get_ne(),
                                            w.get_se(),
                                            e.get_sw());
        let or_value = Arc::new(key.clone());
        hashes.entry(key).or_insert(or_value).clone()
    }

    /// Returns the node representing the south half of n and the north half of s, as if they were
    /// vertically adjacent and lined up, with n on the north side and s on the south. If the
    /// levels of n and s do not match or they are not at least level 1, then the thread panics.
    pub fn vertical_forward(n: Arc<LifeNode>, s: Arc<LifeNode>, hashes: &mut HashMap<LifeNode, Arc<LifeNode>>) -> Arc<LifeNode> {
        assert_eq!(n.get_level(), s.get_level());
        assert!(n.get_level() >= 1);
        let key = LifeNode::with_components(n.get_se(),
                                            n.get_sw(),
                                            s.get_nw(),
                                            s.get_ne());
        let or_value = Arc::new(key.clone());
        hashes.entry(key).or_insert(or_value).clone()
    }
}

struct Life {
    hashes: HashMap<LifeNode, Arc<LifeNode>>,
    advanced_centers: HashMap<LifeNode, Arc<LifeNode>>,
    root: Arc<LifeNode>,
}
