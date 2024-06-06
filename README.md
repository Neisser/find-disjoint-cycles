# find-disjoint-cycles

## Overview

`find-disjoint-cycles` is a Rust library designed to find disjoint cycles in an Eulerian graph. An Eulerian graph is a graph in which every vertex has an even degree. This library provides a Rust implementation to identify and extract disjoint cycles from such graphs.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Algorithm Explanation](#algorithm-explanation)
- [Contributing](#contributing)
- [License](#license)

## Installation

To use this library, add `find-disjoint-cycles` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
find-disjoint-cycles = "0.1.0"
```

Then, import it in your Rust code:

```rust
extern crate find_disjoint_cycles;
```

## Usage

Here is a simple example of how to use the library to find disjoint cycles in an Eulerian graph:

```rust
use find_disjoint_cycles::{Graph, Vertex, Edges};

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

    let disjoint_cycles = find_disjoint_cycles::find_disjunctive_cycle(graph);
    
    for cycle in disjoint_cycles {
        println!("{:?}", cycle);
    }
}
```

## Algorithm Explanation

The algorithm to find disjoint cycles in an Eulerian graph works as follows:

1. **Initialize**:
    - Calculate the total number of edges in the graph.
    - Determine the length of each cycle, which is the total number of edges divided by 3 (assuming we want cycles of equal length).
    - Create a queue to hold the cycle lengths.

2. **Queue Cycles**:
    - Iterate through the edges in steps of the calculated cycle length.
    - If the remaining edges are greater than the cycle length, add them to the queue.
    - Enqueue the cycle length for processing.

3. **Process Queue**:
    - While the queue is not empty:
        - Extract a subgraph based on the current cycle length.
        - Subtract the subgraph from the original graph.
        - Dequeue the processed cycle length.

Here is the pseudocode of the algorithm:

```pseudocode
fn find_disjunctive_cycle(G: graph) {
  num_edges = G.edges
  cycle_len = num_edges / 3
  qc = [] // cycles queue

  for i=0; i<num_edges; i+=cycle_len {
    if num_edges - i > cycle_len {
      qc.enqueue(num_edges - i)
    }

    qc.enqueue(cycle_len)
  }

  while qc.isM_not_empty() {
    get_sub_graph(G, qc[0])
    G.subtrack(sub_graph)
    qc.dequeue()
  }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
