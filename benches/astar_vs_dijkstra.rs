#![feature(test)]

extern crate test;
extern crate petgraph;

use test::Bencher;

use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use petgraph::algo::{astar, dijkstra};

use std::collections::BTreeMap;

/// A graph with no walls, the finish being at a straight line from the start at the opposite of
/// the graph.
const EMPTY: &'static str = "
 S . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . .
 . . . . . . . . . F
";

/// A graph with a box around the start, the finish being very close to the start.
const BOXED: &'static str = "
 . . . . . . . . . F
 . # # # # # # # # .
 . # . . . . . S # .
 . # . . . . . . # .
 . # . . . . . . # .
 . # . . . . . . # .
 . # . . . . . . # .
 . . . . . . . . # .
 . . . # # # # # # .
 . . . . . . . . . .
";

/// A graph with a maze from start to finish.
const MAZE: &'static str = "
 S . . # # # # # # #
 . # . # # . . . . .
 . # . . # . # . # .
 . # # . # . # . # .
 . # # . . . # . # .
 . # # # # # # . # .
 . # . # # . . . # .
 . # . . . . # # # .
 . # # # # # # # # #
 . . . . . . . . . F
";

type Position = (i32, i32);

#[derive(Copy, Clone, Eq, PartialEq)]
enum NodeType {
    Start,
    Finish,
    Empty,
    Wall,
}

type Grid = Graph<(Position, NodeType), (), Undirected>;

/// Parse a grid format into a graph, setting node weights to be:
/// - the node's 2d position
/// - the node's type: start, finish, empty or wall
fn parse_grid(s: &str) -> (Grid, (NodeIndex, NodeIndex)) {
    let mut g = Grid::default();
    let mut start = None;
    let mut finish = None;

    let lines = s.trim().lines().filter(|l| !l.is_empty());
    for (row, line) in lines.enumerate() {
        for (col, word) in line.split(' ')
                                .filter(|s| s.len() > 0)
                                .enumerate()
        {
            let position = (col as i32, row as i32);
            let node_type = match word {
                "." => NodeType::Empty,
                "#" => NodeType::Wall,
                "S" => NodeType::Start,
                "F" => NodeType::Finish,
                _   => unreachable!(),
            };

            let node = g.add_node((position, node_type));

            if node_type == NodeType::Start {
                assert!(start.is_none());
                start = Some(node);
            } else if node_type == NodeType::Finish {
                assert!(finish.is_none());
                finish = Some(node);
            }
        }
    }

    // note: walls are filtered therefore should not appear when searching for neighbors
    let nodes_by_position: BTreeMap<Position, NodeIndex> = g.node_references()
        .filter(|&(_, &(_, node_type))| node_type != NodeType::Wall)
        .map(|(node, &(position, _))| (position, node))
        .collect();

    for (&(x0, y0), &node) in nodes_by_position.iter() {
        for x in x0-1..x0+1 {
            for y in y0-1..y0+1 {
                if x == x0 || y == y0 {
                    continue;
                }

                if let Some(&neighbor) = nodes_by_position.get(&(x, y)) {
                    g.update_edge(node, neighbor, ());
                }
            }
        }
    }

    (g, (start.unwrap(), finish.unwrap()))
}

fn manhattan_distance((x0, y0): Position, (x1, y1): Position) -> i32 {
    let (dx, dy) = (x1 - x0, y1 - y0);
    dx.pow(2) + dy.pow(2)
}

fn distance_to_finish((start, node_type): (Position, NodeType), finish: Position) -> i32 {
    if node_type == NodeType::Wall {
        std::i32::MAX
    } else {
        manhattan_distance(start, finish)
    }
}

fn bench_astar(bencher: &mut Bencher, s: &str) {
    let (g, (start, finish)) = parse_grid(s);

    bencher.iter(|| {
        astar(&g, start, |node| node == finish, |_| 1, |node| {
            distance_to_finish(g[node], g[finish].0)
        })
    })
}

fn bench_dijkstra(bencher: &mut Bencher, s: &str) {
    let (g, (start, finish)) = parse_grid(s);

    bencher.iter(|| {
        dijkstra(&g, start, Some(finish), |_| 1)
    })
}

#[bench]
fn astar_empty_bench(bencher: &mut Bencher) {
    bench_astar(bencher, EMPTY)
}

#[bench]
fn dijkstra_empty_bench(bencher: &mut Bencher) {
    bench_dijkstra(bencher, EMPTY)
}

#[bench]
fn astar_boxed_bench(bencher: &mut Bencher) {
    bench_astar(bencher, BOXED)
}

#[bench]
fn dijkstra_boxed_bench(bencher: &mut Bencher) {
    bench_dijkstra(bencher, BOXED)
}

#[bench]
fn astar_maze_bench(bencher: &mut Bencher) {
    bench_astar(bencher, MAZE)
}

#[bench]
fn dijkstra_maze_bench(bencher: &mut Bencher) {
    bench_dijkstra(bencher, MAZE)
}
