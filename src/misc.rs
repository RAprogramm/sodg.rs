// SPDX-FileCopyrightText: Copyright (c) 2022-2025 Objectionary.com
// SPDX-License-Identifier: MIT

use crate::{BRANCH_NONE, Sodg};

impl<const N: usize> Sodg<N> {
    /// Get total number of vertices in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.vertices
            .iter()
            .filter(|(_, vertex)| vertex.branch != BRANCH_NONE)
            .count()
    }

    /// Is it empty?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self
            .vertices
            .iter()
            .any(|(_, vertex)| vertex.branch != BRANCH_NONE)
    }

    /// Get keys of all vertices alive?
    #[must_use]
    pub fn keys(&self) -> Vec<usize> {
        self.vertices
            .iter()
            .filter_map(|(vertex_id, vertex)| (vertex.branch != BRANCH_NONE).then_some(vertex_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_vertices() {
        let g: Sodg<16> = Sodg::empty(256);
        assert_eq!(0, g.len());
    }

    #[test]
    fn reports_emptiness() {
        let mut g: Sodg<16> = Sodg::empty(256);
        assert!(g.is_empty());
        g.add(0);
        assert!(!g.is_empty());
    }

    #[test]
    fn collects_alive_keys() {
        let mut g: Sodg<16> = Sodg::empty(256);
        g.add(0);
        g.add(2);
        let mut keys = g.keys();
        keys.sort_unstable();
        assert_eq!(vec![0, 2], keys);
    }
}
