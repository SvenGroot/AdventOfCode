use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub trait Graph<Vertex>
where
    Vertex: Copy + Eq + Hash,
{
    fn vertices(&self) -> HashSet<Vertex>;
    fn neighbors(&self, v: &Vertex) -> Vec<Vertex>;
}

pub fn shortest_path<Vertex>(
    graph: &impl Graph<Vertex>,
    source: &Vertex,
    dest: &Vertex,
) -> Vec<Vertex>
where
    Vertex: Copy + Eq + Hash,
{
    let info = shortest_paths_core(graph, source, Some(*dest));
    let mut path = Vec::new();
    if info[dest].previous.is_some() || *dest == *source {
        let mut current = Some(*dest);
        while current.is_some() {
            let vertex = current.unwrap();
            path.push(vertex);
            current = info[&vertex].previous;
        }

        path.reverse();
    }

    path
}

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
    let mut vertices = graph.vertices();
    let mut info = HashMap::new();
    for v in &vertices {
        info.insert(*v, VertexInfo::<Vertex>::default());
    }

    info.get_mut(source).unwrap().distance = 0;
    while !vertices.is_empty() {
        let closest = *vertices.iter().min_by_key(|v| info[v].distance).unwrap();
        if Some(closest) == dest {
            break;
        }

        vertices.remove(&closest);
        let distance = info[&closest].distance;
        if distance == usize::MAX {
            break;
        }

        for neighbor in graph.neighbors(&closest) {
            if vertices.contains(&neighbor) {
                let alt = distance + 1; // No weight support
                let neighbor_info = info.get_mut(&neighbor).unwrap();
                if alt < neighbor_info.distance {
                    neighbor_info.distance = alt;
                    neighbor_info.previous = Some(closest);
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
    pub previous: Option<Vertex>,
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
