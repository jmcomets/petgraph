#[macro_use]
extern crate petgraph;

use petgraph::Graph;

#[test]
fn define_edges_without_weights() {
    let g = define_edges!(Graph,
        0 => 1,
        1 => 2,
        1 => 3,
        2 => 3
    );

    let mut node_weights = g.node_indices().map(|n| {
        let w = *g.node_weight(n).unwrap();

        let mut neighbor_weights = g.neighbors(n)
            .map(|p| { *g.node_weight(p).unwrap() })
            .collect::<Vec<i32>>();
        neighbor_weights.sort();

        (w, neighbor_weights)

    }).collect::<Vec<(i32, Vec<i32>)>>();
    node_weights.sort();

    assert_eq!(node_weights,
               vec![(0, vec![1]),
                    (1, vec![2, 3]),
                    (2, vec![3]),
                    (3, vec![])]);
}
