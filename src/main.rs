mod graph;
mod queue;

use std::vec;

use graph::{Edges, Graph, Vertex};

/// The main function is the entry point of the program.
/// It creates a graph with vertices and edges, displays the graph,
/// and then finds disjoint cycles in the graph.
fn main() {
    // Define the vertices of the graph.
    let vertex: Vertex = vec![0, 1, 2, 3, 4, 5, 6];
    
    // Define the edges of the graph as pairs of vertices.
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

    // Create a new graph using the vertices and edges defined above.
    let graph = Graph::new(vertex, edges);
    
    // Display the graph's adjacency list.
    graph.display();

    // Find and display disjoint cycles in the graph.
    graph.find_disjoint_cycles();
}
