use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait VertexKey: PartialEq + Eq + Hash + Clone {}
impl VertexKey for String {}

pub trait Sizable {
    fn len(&self) -> usize;
}

pub trait Graph<K: VertexKey> {
    fn contains(&self, vertex_key: &K) -> bool;
    fn add_edge(&mut self, from: &K, to: &K);
    fn iter_vertices<'a>(&'a self) -> impl Iterator<Item = &'a K>
    where
        K: 'a;
    fn iter_adjacents<'a>(&'a self, vertex_key: &K) -> Option<impl Iterator<Item = &'a K>>
    where
        K: 'a;
}

pub struct SparseGraph<K: VertexKey> {
    adjacents: HashMap<K, HashSet<K>>,
}

impl<K: VertexKey> SparseGraph<K> {
    pub fn new() -> SparseGraph<K> {
        SparseGraph {
            adjacents: HashMap::new(),
        }
    }
    fn add_undirected_edge(&mut self, a: &K, b: &K) {
        self.add_edge(a, b);
        self.add_edge(b, a);
    }
}

impl<K: VertexKey> Sizable for SparseGraph<K> {
    fn len(&self) -> usize {
        self.adjacents.len()
    }
}

impl<K: VertexKey> Graph<K> for SparseGraph<K> {
    fn contains(&self, vertex_key: &K) -> bool {
        self.adjacents.contains_key(vertex_key)
    }

    fn add_edge(&mut self, from: &K, to: &K) {
        let from_ref = self.adjacents.entry(from.clone()).or_default();
        from_ref.insert(to.clone());

        self.adjacents.entry(to.clone()).or_default();
    }

    fn iter_vertices<'a>(&'a self) -> impl Iterator<Item = &'a K>
    where
        K: 'a,
    {
        self.adjacents.iter().map(|(v, _adjacents)| v)
    }

    fn iter_adjacents<'a>(&'a self, vertex_key: &K) -> Option<impl Iterator<Item = &'a K>>
    where
        K: 'a,
    {
        Some(self.adjacents.get(vertex_key)?.iter())
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for SparseGraph<String> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let mut g = SparseGraph::new();
        for pair in iter.into_iter() {
            g.add_undirected_edge(&pair.0.to_string(), &pair.1.to_string());
        }
        g
    }
}

mod tests_graph {
    use std::collections::HashSet;

    use crate::graph::{Graph, Sizable, SparseGraph};

    #[test]
    fn test_sparse_graph_example() {
        let g: SparseGraph<String> = vec![
            ("start", "A"),
            ("start", "b"),
            ("A", "c"),
            ("A", "b"),
            ("b", "d"),
            ("A", "end"),
            ("b", "end"),
        ]
        .into_iter()
        .collect();

        assert_eq!(g.len(), 6);
        assert_eq!(
            g.iter_vertices()
                .map(|x| x.as_str())
                .collect::<HashSet<_>>(),
            HashSet::from_iter(vec!["start", "A", "b", "c", "d", "end"].into_iter())
        );
    }
}
