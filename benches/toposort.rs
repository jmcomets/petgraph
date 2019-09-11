#![feature(test)]

extern crate test;
extern crate petgraph;

use petgraph::prelude::*;
use petgraph::algo::{
    toposort,
    topology_sorting::{
        Topology,
        TopologySorting
    }
};

fn test_graph() -> DiGraph<&'static str, f64> {
    let mut gr = DiGraph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");
    let f = gr.add_node("F");
    let g = gr.add_node("G");
    gr.extend_with_edges(&[
        (a, b, 7.),
        (a, d, 5.),
        (d, b, 9.),
        (b, c, 8.),
        (b, e, 7.),
        (c, e, 5.),
        (d, e, 15.),
        (d, f, 6.),
        (f, e, 8.),
        (f, g, 11.),
        (e, g, 9.),
    ]);

    // add a disjoint part
    let h = gr.add_node("H");
    let i = gr.add_node("I");
    let j = gr.add_node("J");
    gr.add_edge(h, i, 1.);
    gr.add_edge(h, j, 3.);
    gr.add_edge(i, j, 1.);

    gr
}

#[bench]
fn bench_toposort(bench: &mut test::Bencher) {
    let gr = test_graph();

    bench.iter(|| {
        toposort(&gr, None).unwrap();
    })
}

#[bench]
fn bench_new_toposort(bench: &mut test::Bencher) {
    let gr = test_graph();

    bench.iter(|| {
        gr.get_ordered_list()
    })
}
