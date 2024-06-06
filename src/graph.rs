use std::collections::{HashMap, HashSet};
use crate::queue::Queue;

pub type Vertex = Vec<u32>;
pub type Edges = Vec<Vec<u32>>;
pub type Path = Vec<(u32, u32)>;

#[derive(Debug, Clone)]
/// A structure representing a graph with vertices and edges.
pub struct Graph {
    pub vertex: Vertex,
    pub edges: Edges,
    pub adj: HashMap<u32, Vec<u32>>,
}

impl Graph {
    /// Creates a new graph from the given vertices and edges.
    /// Builds the adjacency list for the graph.
    ///
    /// # Arguments
    ///
    /// * `vertex` - A vector of vertices.
    /// * `edges` - A vector of edges, where each edge is represented by a pair of vertices.
    pub fn new(vertex: Vertex, edges: Edges) -> Graph {
        let mut g: Graph = Graph {
            vertex,
            edges,
            adj: HashMap::new(),
        };
        g.build_adj_list();
        g
    }

    /// Adds an edge between two vertices in the graph.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting vertex of the edge.
    /// * `to` - The ending vertex of the edge.
    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.adj.entry(from).or_insert(Vec::new()).push(to);
        self.adj.entry(to).or_insert(Vec::new()).push(from);
    }

    /// Removes an edge between two vertices in the graph.
    /// If a vertex has no more connections after the removal, it is also removed from the vertex list.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting vertex of the edge.
    /// * `to` - The ending vertex of the edge.
    pub fn remove_edge(&mut self, from: u32, to: u32) {
        if let Some(neighbors) = self.adj.get_mut(&from) {
            neighbors.retain(|&x| x != to);
            if neighbors.is_empty() {
                self.adj.remove(&from);
                self.vertex.retain(|&x| x != from);
            }
        }
        if let Some(neighbors) = self.adj.get_mut(&to) {
            neighbors.retain(|&x| x != from);
            if neighbors.is_empty() {
                self.adj.remove(&to);
                self.vertex.retain(|&x| x != to);
            }
        }
    }

    /// Builds the adjacency list from the edges.
    pub fn build_adj_list(&mut self) {
        for edge in self.edges.clone() {
            self.add_edge(edge[0], edge[1]);
        }
    }

    /// Displays the adjacency list of the graph.
    pub fn display(&self) {
        println!("display graph:");
        for (key, value) in &self.adj {
            println!("{}: {:?}", key, value);
        }
    }

    /// Finds and displays disjoint cycles in the graph.
    pub fn find_disjoint_cycles(&self) {
        println!("find_disjoint_cycles");
        let mut subgraph: Graph = self.clone();
        let num_edges = self.edges.len() as u32;
        let min_cycle_size = num_edges / 3;
        println!("min_cycle_size: {}", min_cycle_size);

        let mut q_c: Queue<u32> = Queue::new();

        self.fill_queue(&mut q_c, num_edges, min_cycle_size);

        q_c.display();

        while !q_c.is_empty() {
            let cycle_size = q_c.dequeue().unwrap();
            println!("cycle_size: {}", cycle_size);
            let cycle: Path = self.find_cycle(&mut subgraph, cycle_size);
            println!("cycle: {:?}", cycle);

            for edge in cycle.clone() {
                subgraph.remove_edge(edge.0, edge.1);
            }
        }
    }

    /// Fills the queue with cycle sizes.
    ///
    /// # Arguments
    ///
    /// * `q_c` - A mutable reference to a queue to store cycle sizes.
    /// * `num_edges` - The total number of edges in the graph.
    /// * `min_cycle_size` - The minimum size of a cycle.
    pub fn fill_queue(&self, q_c: &mut Queue<u32>, num_edges: u32, min_cycle_size: u32) {
        let mut aux_total_edges: u32 = 0;

        for i in vec![min_cycle_size; (num_edges / min_cycle_size) as usize] {
            match aux_total_edges + min_cycle_size > num_edges {
                true => q_c.enqueue(num_edges - aux_total_edges),
                false => q_c.enqueue(i),
            }

            aux_total_edges += min_cycle_size;
        }
    }

    /// Finds a cycle of the specified size in the graph.
    ///
    /// # Arguments
    ///
    /// * `g` - A mutable reference to the graph.
    /// * `cycle_size` - The size of the cycle to find.
    ///
    /// # Returns
    ///
    /// A vector of edges representing the cycle.
    pub fn find_cycle(&self, g: &mut Graph, cycle_size: u32) -> Path {
        /// A helper function for depth-first search to find cycles.
        ///
        /// # Arguments
        ///
        /// * `graph` - A reference to the graph.
        /// * `v` - The current vertex.
        /// * `start` - The starting vertex of the cycle.
        /// * `depth` - The current depth of the search.
        /// * `visited` - A mutable reference to a set of visited vertices.
        /// * `path` - A mutable reference to the current path.
        /// * `cycle_size` - The size of the cycle to find.
        ///
        /// # Returns
        ///
        /// A boolean indicating whether a cycle of the specified size was found.
        fn dfs(
            graph: &Graph,
            v: u32,
            start: u32,
            depth: u32,
            visited: &mut HashSet<u32>,
            path: &mut Path,
            cycle_size: u32,
        ) -> bool {
            if depth == cycle_size - 1 {
                if graph.adj[&v].contains(&start) {
                    path.push((v, start));
                    return true;
                } else {
                    return false;
                }
            }

            visited.insert(v);

            for &neighbor in &graph.adj[&v] {
                if !visited.contains(&neighbor) || (depth == cycle_size - 1 && neighbor == start) {
                    path.push((v, neighbor));
                    if dfs(graph, neighbor, start, depth + 1, visited, path, cycle_size) {
                        return true;
                    }
                    path.pop();
                }
            }

            visited.remove(&v);
            false
        }

        for &start in &g.vertex {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            if dfs(g, start, start, 0, &mut visited, &mut path, cycle_size) {
                return path;
            }
        }

        Vec::new() // return an empty path if no cycle is found
    }
}
