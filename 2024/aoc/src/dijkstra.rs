use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use priority_queue::PriorityQueue;

pub trait Graph<Vertex>
where
    Vertex: Copy + Eq + Hash,
{
    /// Gets a set of all vertices in the graph.
    fn vertices(&self) -> HashSet<Vertex>;

    /// Gets all the neighbors and their weights for a particular vertex in the graph.
    fn neighbors(&self, v: &Vertex) -> Vec<(Vertex, usize)>;
}

/// Get the shortest path from source to dest.
pub fn shortest_path<Vertex>(
    graph: &impl Graph<Vertex>,
    source: &Vertex,
    dest: &Vertex,
) -> Vec<Vertex>
where
    Vertex: Copy + Eq + Hash,
{
    let info = shortest_paths_core(graph, source, Some(*dest));
    path_from_info(&info, dest)
}

pub fn path_from_info<Vertex>(
    info: &HashMap<Vertex, VertexInfo<Vertex>>,
    dest: &Vertex,
) -> Vec<Vertex>
where
    Vertex: Copy + Eq + Hash,
{
    // Walk the path from dest to source and then reverse it.
    let mut path = Vec::new();
    if info[dest].previous.is_some() {
        let mut current = Some(*dest);
        while let Some(vertex) = current {
            path.push(vertex);
            current = info[&vertex].previous;
        }

        path.reverse();
    }

    path
}

/// Get the shortest parts from source to all other reachable vertices.
pub fn shortest_paths<Vertex>(
    graph: &impl Graph<Vertex>,
    source: &Vertex,
) -> HashMap<Vertex, VertexInfo<Vertex>>
where
    Vertex: Copy + Eq + Hash,
{
    shortest_paths_core(graph, source, None)
}

fn shortest_paths_core<Vertex>(
    graph: &impl Graph<Vertex>,
    source: &Vertex,
    dest: Option<Vertex>,
) -> HashMap<Vertex, VertexInfo<Vertex>>
where
    Vertex: Copy + Eq + Hash,
{
    let vertices_source = graph.vertices();
    // println!("{}", vertices_source.len());
    let mut vertices = PriorityQueue::new();
    let mut info = HashMap::new();
    for v in vertices_source.into_iter() {
        let distance = if v == *source { 0 } else { usize::MAX };

        vertices.push(v, Reverse(distance));
        info.insert(
            v,
            VertexInfo::<Vertex> {
                distance,
                previous: None,
            },
        );
    }

    assert!(info.contains_key(source));
    while let Some((closest, distance)) = vertices.pop() {
        if Some(closest) == dest {
            break;
        }

        // if vertices.len() % 1000 == 0 {
        //     println!("{} remaining", vertices.len());
        // }

        if distance.0 == usize::MAX {
            break;
        }

        for (neighbor, weight) in graph.neighbors(&closest) {
            if vertices.get(&neighbor).is_some() {
                let alt = distance.0 + weight;
                let neighbor_info = info.get_mut(&neighbor).unwrap();
                if alt < neighbor_info.distance {
                    neighbor_info.distance = alt;
                    neighbor_info.previous = Some(closest);
                    vertices.change_priority(&neighbor, Reverse(alt));
                }
            }
        }
    }

    info
}

pub struct VertexInfo<Vertex>
where
    Vertex: Copy + Eq + Hash,
{
    // The previous step on the path from `source` to this vertex.
    pub previous: Option<Vertex>,
    // The distance from source to this vertex.
    pub distance: usize,
}

impl<Vertex> Default for VertexInfo<Vertex>
where
    Vertex: Copy + Eq + Hash,
{
    fn default() -> Self {
        Self {
            previous: None,
            distance: usize::MAX,
        }
    }
}
