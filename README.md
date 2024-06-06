# find-disjoint-cycles

## Overview

`find-disjoint-cycles` is a Rust library designed to find disjoint cycles in an Eulerian graph. An Eulerian graph is a graph in which every vertex has an even degree. This library provides a Rust implementation to identify and extract disjoint cycles from such graphs.

The project consists of two main modules:

1. `Graph`: Represents a graph with vertices and edges.
2. `Queue`: Represents a generic queue structure.

## Graph Module

The `Graph` struct represents a graph with vertices and edges, and provides methods to manage edges, find cycles, and display the graph.

### Graph Methods

- `new(vertex: Vertex, edges: Edges) -> Graph`: Creates a new graph from given vertices and edges, and builds the adjacency list.
- `add_edge(&mut self, from: u32, to: u32)`: Adds an edge between two vertices.
- `remove_edge(&mut self, from: u32, to: u32)`: Removes an edge between two vertices. If a vertex has no more connections after the removal, it is also removed from the vertex list.
- `build_adj_list(&mut self)`: Builds the adjacency list from the edges.
- `display(&self)`: Displays the adjacency list of the graph.
- `find_disjoint_cycles(&self)`: Finds and displays disjoint cycles in the graph. It clones the graph to avoid modifying the original graph.
- `fill_queue(&self, q_c: &mut Queue<u32>, num_edges: u32, min_cycle_size: u32)`: Fills a queue with cycle sizes.
- `find_cycle(&self, g: &mut Graph, cycle_size: u32) -> Path`: Finds a cycle of a specified size in the graph using depth-first search (DFS).

### Example

```rust
mod graph;
mod queue;

use std::vec;

use graph::{Edges, Graph, Vertex};

fn main() {
    let vertex: Vertex = vec![0, 1, 2, 3, 4, 5, 6];
    let edges: Edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 5],
        vec![3, 4],
        vec![3, 5],
        vec![4, 5],
        vec![4, 6],
        vec![5, 6],
    ];

    let graph = Graph::new(vertex, edges);
    graph.display();
    graph.find_disjoint_cycles();
}
```

### How `find_disjoint_cycles` Works

The `find_disjoint_cycles` function aims to find and display disjoint cycles in the graph. Here's a step-by-step explanation of how it works:

1. **Clone the Graph**: The function begins by cloning the original graph to ensure that the original graph remains unmodified during the cycle-finding process.

    ```rust
    let mut subgraph: Graph = self.clone();
    ```

2. **Calculate Minimum Cycle Size**: It calculates the minimum cycle size as one-third of the total number of edges.

    ```rust
    let num_edges = self.edges.len() as u32;
    let min_cycle_size = num_edges / 3;
    ```

3. **Fill the Queue**: A queue is filled with cycle sizes. The `fill_queue` method ensures that the total cycle sizes match the number of edges in the graph.

    ```rust
    let mut q_c: Queue<u32> = Queue::new();
    self.fill_queue(&mut q_c, num_edges, min_cycle_size);
    ```

4. **Process Each Cycle Size**: While the queue is not empty, the function dequeues a cycle size and attempts to find a cycle of that size in the graph.

    ```rust
    while !q_c.is_empty() {
        let cycle_size = q_c.dequeue().unwrap();
        let cycle: Path = self.find_cycle(&mut subgraph, cycle_size);
        for edge in cycle.clone() {
            subgraph.remove_edge(edge.0, edge.1);
        }
    }
    ```

5. **Find Cycle**: The `find_cycle` method uses a depth-first search (DFS) approach to find a cycle of the specified size. If a cycle is found, it is returned as a path. If no cycle is found, an empty path is returned.

    ```rust
    pub fn find_cycle(&self, g: &mut Graph, cycle_size: u32) -> Path {
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
    ```

6. **Remove Cycle Edges**: For each found cycle, the edges of the cycle are removed from the subgraph to ensure the cycles are disjoint.

    ```rust
    for edge in cycle.clone() {
        subgraph.remove_edge(edge.0, edge.1);
    }
    ```

This function iteratively finds cycles of specified sizes and removes them from the graph until no more cycles can be found or the queue is empty.

## Queue Module

The `Queue` struct represents a generic queue structure for holding items of any type that implements the `Debug` trait.

### Queue Methods

- `new() -> Queue<T>`: Creates a new, empty queue.
- `enqueue(&mut self, item: T)`: Adds an item to the end of the queue.
- `dequeue(&mut self) -> Option<T>`: Removes and returns the item from the front of the queue. If the queue is empty, it returns `None`.
- `is_empty(&self) -> bool`: Checks if the queue is empty and returns `true` if it is, or `false` otherwise.
- `display(&self)`: Prints the contents of the queue to the console.

### Example

```rust
use std::fmt::Debug;

pub struct Queue<T: Debug> {
    pub items: Vec<T>,
}

impl<T: Debug> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn display(&self) {
        println!("queue: {:?}", self.items);
    }
}
```

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/graph-cycle-finder.git
    ```

3. Navigate to the project directory:

    ```sh
    cd graph-cycle-finder
    ```

4. Build the project:

    ```sh
    cargo build
    ```

5. Run the project:

    ```sh
    cargo run
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
