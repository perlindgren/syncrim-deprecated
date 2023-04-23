use petgraph::algo;
use petgraph::algo::toposort;
use petgraph::Graph;

#[derive(Default)] // required by Graph
struct Node {
    value: u8,
}

fn main() {
    let mut g1 = Graph::<_, (), petgraph::Directed>::new();

    let n0 = g1.add_node(Node { value: 43 });
    let n1 = g1.add_node(Node { value: 44 });
    let n2 = g1.add_node(Node { value: 45 });
    let n3 = g1.add_node(Node { value: 46 });

    g1.extend_with_edges(&[(n1, n3, ()), (n0, n1, ()), (n0, n2, ()), (n0, n3, ())]);

    let top = toposort(&g1, None);
    println!("{:?}", top);
}
