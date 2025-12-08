use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }

        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in the same set
        }

        // Union by rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        true
    }

    pub fn count_components(&mut self) -> usize {
        let mut roots = HashSet::new();

        for i in 0..self.parent.len() {
            roots.insert(self.find(i)); // Find root of each element
        }

        roots.len()
    }

    pub fn get_component_sizes(&mut self) -> HashMap<usize, usize> {
        let mut sizes = HashMap::new();

        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }

        sizes
    }
}
