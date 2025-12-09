use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
};

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,

    // Used for part 1
    pub component_lengths: Vec<usize>,
    // Used for part 2
    pub components_count: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            components_count: size,
            component_lengths: vec![1; size],
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
            Less => {
                self.parent[root_x] = root_y;
                self.component_lengths[root_y] += self.component_lengths[root_x];
                self.component_lengths[root_x] = 0;
            }
            Greater => {
                self.parent[root_y] = root_x;
                self.component_lengths[root_x] += self.component_lengths[root_y];
                self.component_lengths[root_y] = 0;
            }
            Equal => {
                self.parent[root_y] = root_x;
                self.component_lengths[root_x] += self.component_lengths[root_y];
                self.component_lengths[root_y] = 0;
                self.rank[root_x] += 1;
            }
        }
        self.components_count -= 1;

        true
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
