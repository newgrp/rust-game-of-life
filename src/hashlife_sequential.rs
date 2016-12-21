use std::sync::Arc;
use std::collections::HashSet;

enum LifeData {
    Leaf { alive: bool },
    Split { ne: Arc<LifeNode>, nw: Arc<LifeNode>, sw: Arc<LifeNode>, se: Arc<LifeNode> },
}

use self::LifeData::*;

struct LifeNode {
    level: u64,
    info: LifeData,
}

struct Life {
    hashes: HashSet<Arc<LifeNode>>,
    root: Arc<LifeNode>,
}
