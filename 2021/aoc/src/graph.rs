use std::ops::{Index, IndexMut};

use crate::dijkstra;

#[derive(Clone)]
pub struct GraphVertex<T> {
    value: T,
    neighbors: Vec<VertexId>,
}

impl<T> GraphVertex<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn neighbors(&self) -> &[VertexId] {
        &self.neighbors
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexId(usize);

#[derive(Clone)]
pub struct Graph<T>(Vec<GraphVertex<T>>);

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn add_vertex(&mut self, value: T) -> VertexId {
        self.0.push(GraphVertex {
            value,
            neighbors: Vec::new(),
        });

        VertexId(self.0.len() - 1)
    }

    pub fn add_edge_by_id(&mut self, from: VertexId, to: VertexId) {
        assert!(to.0 < self.0.len());
        self.0[from.0].neighbors.push(to);
    }

    pub fn find_vertex<U>(&self, value: &U) -> Option<VertexId>
    where
        U: ?Sized,
        T: PartialEq<U>,
    {
        self.0
            .iter()
            .position(|item| item.value == *value)
            .map(VertexId)
    }

    pub fn add_edge<U>(&mut self, from: &U, to: &U, bidirectional: bool)
    where
        U: ?Sized,
        T: PartialEq<U>,
    {
        let from = self.find_vertex(from).unwrap();
        let to = self.find_vertex(to).unwrap();
        self.add_edge_by_id(from, to);
        if bidirectional {
            self.add_edge_by_id(to, from);
        }
    }
}

impl<T> Index<VertexId> for Graph<T> {
    type Output = GraphVertex<T>;

    fn index(&self, index: VertexId) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<T> IndexMut<VertexId> for Graph<T> {
    fn index_mut(&mut self, index: VertexId) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl<T> dijkstra::Graph<VertexId> for Graph<T> {
    fn vertices(&self) -> std::collections::HashSet<VertexId> {
        (0..self.0.len()).map(VertexId).collect()
    }

    fn neighbors(&self, v: &VertexId) -> Vec<(VertexId, usize)> {
        self.0[v.0].neighbors.iter().map(|nb| (*nb, 1)).collect()
    }
}
