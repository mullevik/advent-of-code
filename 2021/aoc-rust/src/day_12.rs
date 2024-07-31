use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub fn first_part(input: &str) -> i32 {
    unimplemented!()
}

pub fn second_part(input: &str) -> i32 {
    unimplemented!()
}

pub struct SparseGraph<K: PartialEq + Eq + Hash + Clone> {
    adjacents: HashMap<K, HashSet<K>>,
}

impl<K: PartialEq + Eq + Hash + Clone> SparseGraph<K> {
    pub fn new() -> SparseGraph<K> {
        SparseGraph {
            adjacents: HashMap::new(),
        }
    }

    pub fn contains(&self, vertex_key: &K) -> bool {
        self.adjacents.contains_key(vertex_key)
    }

    pub fn len(&self) -> usize {
        self.adjacents.len()
    }

    pub fn add_undirected_edge(&mut self, a: &K, b: &K) {
        self.add_edge(a, b);
        self.add_edge(b, a);
    }

    pub fn add_edge(&mut self, from: &K, to: &K) {
        let from_ref = self.adjacents.entry(from.clone()).or_default();
        from_ref.insert(to.clone());

        self.adjacents.entry(to.clone()).or_default();
    }

    pub fn iter_vertices(&self) -> impl Iterator<Item = &K> {
        self.adjacents.iter().map(|(v, _adjacents)| v)
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

fn parse(input: &str) -> SparseGraph<String> {
    input
        .lines()
        .map(|row| row.split_once("-").unwrap())
        .collect()
}

#[cfg(test)]
mod tests_day_12 {
    use std::collections::HashSet;

    use crate::day_12::parse;

    #[test]
    fn test_parsing() {
        let g = parse(include_str!("../inputs/12_example"));

        assert_eq!(g.len(), 6);
        assert_eq!(
            g.iter_vertices()
                .map(|x| x.as_str())
                .collect::<HashSet<_>>(),
            HashSet::from_iter(vec!["start", "A", "b", "c", "d", "end"].iter().cloned())
        );
    }
}
