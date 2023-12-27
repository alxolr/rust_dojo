use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

/// Point position
pub type Pos = (isize, isize);

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

// We will keep the visited nodes and directions to not get in cycles
pub type PosDir = (Pos, Direction);

#[derive(PartialEq, Eq)]
pub struct Node {
    pub pos: Pos,
    pub dir: Direction,
    // we will store the relaxed from the start to current position
    pub cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

/// We will get the list of possible neighbors using our neighbour function
pub fn dijkstra<F>(start: &Node, grid: &[Vec<usize>], neighbours: F, goal: Pos) -> usize
where
    F: Fn(&Node, &[Vec<usize>]) -> Vec<Node>,
{
    let mut relaxed_nodes: HashMap<Pos, usize> = HashMap::new();
    let mut visited: HashSet<PosDir> = HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push(Reverse(start));

    while let Some(node) = queue.pop() {
    }

    todo!()
}
