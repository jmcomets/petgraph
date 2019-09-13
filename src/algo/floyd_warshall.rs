use std::marker::PhantomData;
use std::ops::Index;

use crate::prelude::*;
use crate::matrix_graph::MatrixGraph;

use crate::algo::{
    FloatMeasure,
};

use crate::visit::{
    Data,
    IntoEdgeReferences,
    IntoNodeIdentifiers,
    NodeCompactIndexable,
    NodeIndexable,
};

pub trait FloydWarshallGraph: NodeCompactIndexable + IntoNodeIdentifiers + IntoEdgeReferences {
}

impl<T: NodeCompactIndexable + IntoNodeIdentifiers + IntoEdgeReferences> FloydWarshallGraph for T {
}

pub struct FloydWarshall<G, T> {
    g: G,
    matrix: MatrixGraph<(), T>, // TODO use bitset with bool output
}

impl<G: FloydWarshallGraph, T> FloydWarshall<G, T> {
    fn new<M>(g: G, m: M) -> Self
        where M: Minimizer<G, Output=T>,
    {
        let matrix = floyd_warshall_impl(g, m);
        FloydWarshall { g, matrix }
    }
}

impl<G: FloydWarshallGraph, T> Index<(G::NodeId, G::NodeId)> for FloydWarshall<G, T> {
    type Output = T;

    fn index(&self, (source, target): (G::NodeId, G::NodeId)) -> &Self::Output {
        let source = self.matrix.from_index(self.g.to_index(source));
        let target = self.matrix.from_index(self.g.to_index(target));
        &self.matrix[(source, target)]
    }
}

impl<G: FloydWarshallGraph> FloydWarshall<G, bool> {
    pub fn transitive_closure(g: G) -> Self {
        FloydWarshall::new(g, BoolMinimizer)
    }
}

impl<G: FloydWarshallGraph, T> FloydWarshall<G, T>
    where G: Data<EdgeWeight=T>,
          T: FloatMeasure
{
    pub fn all_pairs_shortest_path(g: G) -> Self {
        FloydWarshall::new(g, FloatMinimizer::<T>::new())
    }
}

// Algorithm:
//
//   for i = 1 to n
//       for j = 1 to n
//           d[i][j] = { w(i, j) if (i, j) in E;
//                       0       if i == j;
//                       inf.    otherwise }
//
//   for k = 1 to n
//       for i = 1 to n
//           for j = 1 to n
//               d[i][j] = min { d[i][j], d[i][k] + d[k][j] }
//
fn floyd_warshall_impl<G, M>(g: G, _minimizer: M) -> MatrixGraph<(), M::Output>
    where G: NodeCompactIndexable + IntoNodeIdentifiers + IntoEdgeReferences,
          M: Minimizer<G>
{
    let n = g.node_count();
    let mut matrix: MatrixGraph<(), M::Output> = MatrixGraph::with_capacity(n);
    for _ in 0..n {
        let _ = matrix.add_node(());
    }

    // initialize edge weights
    for source in (0..n).map(NodeIndex::new) {
        for target in (0..n).map(NodeIndex::new) {
            let weight = if source != target { M::maximum() } else { M::minimum() };
            matrix.add_edge(source, target, weight);
        }
    }

    // set known weights
    for edge in g.edge_references() {
        let source = matrix.from_index(g.to_index(edge.source()));
        let target = matrix.from_index(g.to_index(edge.target()));
        let weight = M::from_edge_weight(edge.weight());
        let _ = matrix.update_edge(source, target, weight);
    }

    // run
    for link in (0..n).map(NodeIndex::new) {
        for source in (0..n).map(NodeIndex::new) {
            for target in (0..n).map(NodeIndex::new) {
                let new = M::add(&matrix[(source, link)], &matrix[(link, target)]);
                let ref mut current = matrix[(source, target)];
                if M::minimizes(&current, &new) {
                    *current = new;
                }
            }
        }
    }

    matrix
}

trait Minimizer<G: Data> {
    type Output;

    fn from_edge_weight(weight: &G::EdgeWeight) -> Self::Output;

    fn minimum() -> Self::Output;

    fn maximum() -> Self::Output;

    fn add(left: &Self::Output, right: &Self::Output) -> Self::Output;

    fn minimizes(left: &Self::Output, right: &Self::Output) -> bool;
}

struct FloatMinimizer<T: FloatMeasure>(PhantomData<T>);

impl<T: FloatMeasure> FloatMinimizer<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<G: Data<EdgeWeight=T>, T: FloatMeasure> Minimizer<G> for FloatMinimizer<T> {
    type Output = T;

    fn from_edge_weight(weight: &G::EdgeWeight) -> T {
        weight.clone()
    }

    fn minimum() -> T {
        T::zero()
    }

    fn maximum() -> T {
        T::infinite()
    }

    fn add(left: &Self::Output, right: &Self::Output) -> Self::Output {
        *left + *right
    }

    fn minimizes(left: &Self::Output, right: &Self::Output) -> bool {
        left < right
    }
}

struct BoolMinimizer;

impl<G: Data> Minimizer<G> for BoolMinimizer {
    type Output = bool;

    fn from_edge_weight(_: &G::EdgeWeight) -> bool {
        true
    }

    fn minimum() -> bool {
        true
    }

    fn maximum() -> bool {
        false
    }

    fn add(left: &Self::Output, right: &Self::Output) -> Self::Output {
        *left && *right
    }

    fn minimizes(_: &Self::Output, right: &Self::Output) -> bool {
        *right
    }
}
