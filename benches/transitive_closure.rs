#![feature(test)]

extern crate petgraph;
extern crate test;

use test::Bencher;

#[allow(dead_code)]
mod common;
use common::*;

use petgraph::algo;

#[bench]
fn transitive_closure_dfs_graph(bench: &mut Bencher) {
    let g = digraph().bigger();
    bench.iter(|| algo::transitive_closure_dfs(&g));
}

#[bench]
fn transitive_closure_fw_graph(bench: &mut Bencher) {
    let g = digraph().bigger();
    bench.iter(|| algo::transitive_closure_fw(&g));
}

#[bench]
fn transitive_closure_dfs_matrix(bench: &mut Bencher) {
    let g = dimatrix().bigger();
    bench.iter(|| algo::transitive_closure_dfs(&g));
}

#[bench]
fn transitive_closure_fw_matrix(bench: &mut Bencher) {
    let g = dimatrix().bigger();
    bench.iter(|| algo::transitive_closure_fw(&g));
}
