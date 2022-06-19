use crate::common::constants::MAX_SIZE;
use std::iter::Iterator;

#[derive(Copy, Clone)]
pub struct Edge {
    pub next: i32,
    pub to: usize,
}

const MAX_EDGE: usize = 2000;

pub struct Graph {
    pub head: [i32; MAX_SIZE * MAX_SIZE],
    pub edges: [Edge; MAX_EDGE],
    pub edge_count: usize,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            head: [-1; MAX_SIZE * MAX_SIZE],
            edges: [Edge { next: -1, to: 0 }; MAX_EDGE],
            edge_count: 0
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        let e = self.head[from];
        self.edges[self.edge_count].next = e;
        self.edges[self.edge_count].to = to;
        self.head[from] = self.edge_count as i32;
        self.edge_count += 1;
    }

    pub fn iter_edge(&self, point: usize) -> EdgeIterator {
        EdgeIterator {
            graph: &self,
            current: self.head[point]
        }
    }

    pub fn is_neighbor(&self, from: usize, to: usize) -> bool {
        for e in self.iter_edge(from) {
            if e.to == to {
                return true;
            }
        }

        false
    }
}

pub struct EdgeIterator<'a> {
    pub graph: &'a Graph,
    pub current: i32,
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = &'a Edge;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current != -1 {
            let edge = &self.graph.edges[self.current as usize];
            // println!("{}", edge.next);
            self.current = edge.next;
            Some(edge)
        } else {
            None
        }
    }
}