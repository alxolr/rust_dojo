pub struct Edge<T> {
    from: T,
    to: T,
    weight: Option<i32>,
}

impl<T> Edge<T> {
    pub fn new(self, from: T, to: T, weight: Option<i32>) -> Edge<T> {
        Edge { from, to, weight }
    }
}

pub struct Graph<T> {
    vertices: Vec<T>,
    edges: Vec<Edge<T>>,
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            vertices: vec![],
            edges: vec![],
        }
    }

    pub fn add_vertex(mut self, vertex: T) {
        self.vertices.push(vertex);
    }

    pub fn add_edge(mut self, edge: Edge<T>) {
        self.edges.push(edge);
    }

    pub fn from_matrix(matrix: Vec<Vec<T>>) -> Graph<T> {
        let n = matrix.len();
        let m = matrix[0].len();
        let graph = Graph::new();

        for i in 0..n - 1 {
            for j in 0..m - 1 {}
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
