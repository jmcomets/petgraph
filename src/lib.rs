
//! **petgraph** is a graph data structure library.
//!
//! - [`Graph`](./graph/struct.Graph.html) which is an adjacency list graph with
//! arbitrary associated data.
//!
//! - [`StableGraph`](./stable_graph/struct.StableGraph.html) is similar
//! to `Graph`, but it keeps indices stable across removals.
//!
//! - [`GraphMap`](./graphmap/struct.GraphMap.html) is an adjacency list graph
//! which is backed by a hash table and the node identifiers are the keys
//! into the table.
//!
#![doc(html_root_url = "https://docs.rs/petgraph/0.4/")]

extern crate fixedbitset;
#[cfg(feature = "graphmap")]
extern crate ordermap;

use std::fmt;
use std::hash::Hash;

#[doc(no_inline)]
pub use graph::Graph;

pub use Direction::{Outgoing, Incoming};

#[macro_use]
mod macros;
mod scored;

// these modules define trait-implementing macros
#[macro_use]
pub mod visit;
#[macro_use]
pub mod data;

pub mod algo;
#[cfg(feature = "generate")]
pub mod generate;
#[cfg(feature = "graphmap")]
pub mod graphmap;
#[path = "graph.rs"]
mod graph_impl;
pub mod dot;
pub mod unionfind;
mod dijkstra;
pub mod csr;
mod iter_format;
mod isomorphism;
mod traits_graph;
mod util;
#[cfg(feature = "quickcheck")]
mod quickcheck;

pub mod prelude;

/// `Graph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation.
pub mod graph {
    pub use graph_impl::{
        Edge,
        EdgeIndices,
        EdgeReference,
        EdgeReferences,
        EdgeWeightsMut,
        Edges,
        Externals,
        Frozen,
        Graph,
        Neighbors,
        Node,
        NodeIndices,
        NodeWeightsMut,
        WalkNeighbors,
        GraphIndex,
        DiGraph,
        UnGraph,
    };
}

#[cfg(feature = "stable_graph")]
pub use graph_impl::stable_graph;

macro_rules! copyclone {
    ($name:ident) => {
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self { *self }
        }
    }
}

// Index into the NodeIndex and EdgeIndex arrays
/// Edge direction.
#[derive(Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    /// An `Outgoing` edge is an outward edge *from* the current node.
    Outgoing = 0,
    /// An `Incoming` edge is an inbound edge *to* the current node.
    Incoming = 1
}

copyclone!(Direction);

impl Direction {
    /// Return the opposite `Direction`.
    #[inline]
    pub fn opposite(&self) -> Direction {
        match *self {
            Outgoing => Incoming,
            Incoming => Outgoing,
        }
    }

    /// Return `0` for `Outgoing` and `1` for `Incoming`.
    #[inline]
    pub fn index(&self) -> usize {
        (*self as usize) & 0x1
    }
}

#[doc(hidden)]
pub use Direction as EdgeDirection;

/// Marker type for a directed graph.
#[derive(Copy, Debug)]
pub enum Directed { }
copyclone!(Directed);

/// Marker type for an undirected graph.
#[derive(Copy, Debug)]
pub enum Undirected { }
copyclone!(Undirected);

/// A graph's edge type determines whether is has directed edges or not.
pub trait EdgeType {
    fn is_directed() -> bool;
}

impl EdgeType for Directed {
    #[inline]
    fn is_directed() -> bool { true }
}

impl EdgeType for Undirected {
    #[inline]
    fn is_directed() -> bool { false }
}

/// The default integer type for graph indices.
/// `u32` is the default to reduce the size of the graph's data and improve
/// performance in the common case.
///
/// Used for node and edge indices in `Graph` and `StableGraph`, used
/// for node indices in `Csr`.
pub type DefaultIx = u32;

/// Trait for the unsigned integer type used for node and edge indices.
///
/// Marked `unsafe` because: the trait must faithfully preseve
/// and convert index values.
pub unsafe trait IndexType : Copy + Default + Hash + Ord + fmt::Debug + 'static
{
    fn new(x: usize) -> Self;
    fn index(&self) -> usize;
    fn max() -> Self;
}

unsafe impl IndexType for usize {
    #[inline(always)]
    fn new(x: usize) -> Self { x }
    #[inline(always)]
    fn index(&self) -> Self { *self }
    #[inline(always)]
    fn max() -> Self { ::std::usize::MAX }
}

unsafe impl IndexType for u32 {
    #[inline(always)]
    fn new(x: usize) -> Self { x as u32 }
    #[inline(always)]
    fn index(&self) -> usize { *self as usize }
    #[inline(always)]
    fn max() -> Self { ::std::u32::MAX }
}

unsafe impl IndexType for u16 {
    #[inline(always)]
    fn new(x: usize) -> Self { x as u16 }
    #[inline(always)]
    fn index(&self) -> usize { *self as usize }
    #[inline(always)]
    fn max() -> Self { ::std::u16::MAX }
}

unsafe impl IndexType for u8 {
    #[inline(always)]
    fn new(x: usize) -> Self { x as u8 }
    #[inline(always)]
    fn index(&self) -> usize { *self as usize }
    #[inline(always)]
    fn max() -> Self { ::std::u8::MAX }
}

/// Node identifier.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeIndex<Ix=DefaultIx>(Ix);

impl<Ix: IndexType> NodeIndex<Ix>
{
    #[inline]
    pub fn new(x: usize) -> Self {
        NodeIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize
    {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self
    {
        NodeIndex(IndexType::max())
    }

    fn _into_edge(self) -> EdgeIndex<Ix> {
        EdgeIndex(self.0)
    }
}

impl<Ix: IndexType> From<Ix> for NodeIndex<Ix> {
    fn from(ix: Ix) -> Self { NodeIndex(ix) }
}

impl<Ix: fmt::Debug> fmt::Debug for NodeIndex<Ix>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeIndex({:?})", self.0)
    }
}

/// Short version of `NodeIndex::new`
pub fn node_index<Ix: IndexType>(index: usize) -> NodeIndex<Ix> { NodeIndex::new(index) }

/// Short version of `EdgeIndex::new`
pub fn edge_index<Ix: IndexType>(index: usize) -> EdgeIndex<Ix> { EdgeIndex::new(index) }

/// Edge identifier.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EdgeIndex<Ix=DefaultIx>(Ix);

impl<Ix: IndexType> EdgeIndex<Ix>
{
    #[inline]
    pub fn new(x: usize) -> Self {
        EdgeIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize
    {
        self.0.index()
    }

    /// An invalid `EdgeIndex` used to denote absence of an edge, for example
    /// to end an adjacency list.
    #[inline]
    pub fn end() -> Self {
        EdgeIndex(IndexType::max())
    }

    fn _into_node(self) -> NodeIndex<Ix> {
        NodeIndex(self.0)
    }
}

impl<Ix: fmt::Debug> fmt::Debug for EdgeIndex<Ix>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EdgeIndex({:?})", self.0)
    }
}
/*
 * FIXME: Use this impl again, when we don't need to add so many bounds
impl<Ix: IndexType> fmt::Debug for EdgeIndex<Ix>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "EdgeIndex("));
        if *self == EdgeIndex::end() {
            try!(write!(f, "End"));
        } else {
            try!(write!(f, "{}", self.index()));
        }
        write!(f, ")")
    }
}
*/

const DIRECTIONS: [Direction; 2] = [Outgoing, Incoming];

/// Convert an element like `(i, j)` or `(i, j, w)` into
/// a triple of source, target, edge weight.
///
/// For `Graph::from_edges` and `GraphMap::from_edges`.
pub trait IntoWeightedEdge<E> {
    type NodeId;
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E);
}

impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix)
    where E: Default
{
    type NodeId = Ix;

    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = self;
        (s, t, E::default())
    }
}

impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix, E)
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        self
    }
}

impl<'a, Ix, E> IntoWeightedEdge<E> for (Ix, Ix, &'a E)
    where E: Clone
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (a, b, c) = self;
        (a, b, c.clone())
    }
}

impl<'a, Ix, E> IntoWeightedEdge<E> for &'a (Ix, Ix)
    where Ix: Copy, E: Default
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = *self;
        (s, t, E::default())
    }
}

impl<'a, Ix, E> IntoWeightedEdge<E> for &'a (Ix, Ix, E)
    where Ix: Copy, E: Clone
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        self.clone()
    }
}
