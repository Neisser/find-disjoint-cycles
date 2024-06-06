use std::collections::HashMap;

use crate::queue::Queue;

pub type Vertex = Vec<u32>;
pub type Edges = Vec<Vec<u32>>;
pub type Path = Vec<(u32, u32)>;

pub struct Graph {
  pub vertex: Vertex,
  pub edges: Edges,
  pub adj: HashMap<u32, Vec<u32>>
}

impl Graph {
  pub fn new(vertex: Vertex, edges: Edges) -> Graph {
    let mut g: Graph = Graph {
      vertex,
      edges,
      adj: HashMap::new(),
    };

    g.build_adj_list();

    g
  }

  pub fn add_edge(&mut self, from: u32, to: u32) {
    self.adj.entry(from).or_insert(Vec::new()).push(to);
    self.adj.entry(to).or_insert(Vec::new()).push(from);
  }

  pub fn build_adj_list(&mut self) {
    for edge in self.edges.clone(){
      self.add_edge(edge[0], edge[1]);
    }
  }

  pub fn display(&self) {
    println!("display");
    for (key, value) in &self.adj {
      println!("{}: {:?}", key, value);
    }
  }
}