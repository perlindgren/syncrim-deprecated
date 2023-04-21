use petgraph::algo;
use petgraph::algo::toposort;
use petgraph::Graph;

fn main() {
    let mut graph = Graph::<(), ()>::new();

    graph.extend_with_edges(&[(3, 45), (0, 1), (1, 3), (0, 3)]);

    let top = toposort(&graph, None);
    println!("{:?}", top);

    // for start in graph.node_indices() {
    //     println!("--- {:?} ---", start.index());
    //     println!("{:?}", algo::dijkstra(&graph, start, None, |_| 1));
    // }
}
