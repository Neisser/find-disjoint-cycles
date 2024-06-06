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

//   graph.find_disjoint_cycles();
}