#![feature(test)]

extern crate petgraph;
extern crate test;

use petgraph::graph::{Graph, node_index, edge_index};

/// Return a graph with all nodes having a single incoming edge from a root node.
fn root_centric_graph(nb_nodes: usize) -> Graph<(), ()> {
    assert!(nb_nodes > 0);
    let mut g = Graph::new();
    let root = g.add_node(());
    for _ in 0..nb_nodes-1 {
        let n = g.add_node(());
        g.add_edge(root, n, ());
    }
    g
}

/// Add edges going from each node to the node with the next index, similar to a linked list.
fn add_linked_list_edges(g: &mut Graph<(), ()>) {
    for (a, b) in g.node_indices().zip(g.node_indices().skip(1)) {
        g.add_edge(a, b, ());
    }
}

/// Return a graph with all nodes having a single incoming edge from a previous node, similar to a
/// linked list.
fn linked_list_graph(nb_nodes: usize) -> Graph<(), ()> {
    let mut g = Graph::new();
    for _ in 0..nb_nodes {
        g.add_node(());
    }
    add_linked_list_edges(&mut g);
    g
}

#[bench]
fn add_node(b: &mut test::Bencher) {
    let g = root_centric_graph(512);
    b.iter(|| {
        let mut g = g.clone();
        for _ in 0..128 {
            g.add_node(());
        }
    })
}

#[bench]
fn add_edge(b: &mut test::Bencher) {
    let mut g = Graph::new();
    for _ in 0..512 {
        g.add_node(());
    }

    b.iter(|| {
        let mut g = g.clone();
        add_linked_list_edges(&mut g);
    })
}

// Removal is very slow in a big graph, and this one doesn't even have many nodes.
#[bench]
fn remove_node(b: &mut test::Bencher) {
    let g = linked_list_graph(512);
    b.iter(|| {
        let mut g = g.clone();
        for i in 0..128 {
            g.remove_node(node_index(i));
        }
    })
}

#[bench]
fn remove_edge(b: &mut test::Bencher) {
    let g = linked_list_graph(512);
    b.iter(|| {
        let mut g = g.clone();
        for i in 0..128 {
            g.remove_edge(edge_index(i));
        }
    })
}

#[bench]
fn clone_root_centric(b: &mut test::Bencher) {
    let g = root_centric_graph(512);
    b.iter(|| {
        let _ = g.clone();
    });
}

#[bench]
fn clone_linked_list(b: &mut test::Bencher) {
    let g = linked_list_graph(512);
    b.iter(|| {
        let _ = g.clone();
    });
}
