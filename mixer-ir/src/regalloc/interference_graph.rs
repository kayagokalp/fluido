use std::collections::{HashMap, HashSet};

use petgraph::prelude::UnGraph;

#[derive(Debug)]
pub struct InterferenceGraphBuilder<'a> {
    liveness_analysis: &'a [HashSet<usize>],
}

pub struct InterferenceGraph {
    graph: UnGraph<usize, ()>,
}

impl InterferenceGraph {
    pub fn new(graph: UnGraph<usize, ()>) -> Self {
        Self { graph }
    }
    pub fn dot(&self) -> String {
        format!("{:?}", petgraph::dot::Dot::new(&self.graph))
    }
}

impl<'a> InterferenceGraphBuilder<'a> {
    pub fn new(liveness_analysis: &'a [HashSet<usize>]) -> Self {
        Self { liveness_analysis }
    }

    fn number_of_variables_used(&self) -> usize {
        let mut all_vars: HashSet<usize> = HashSet::new();
        for var_set in self.liveness_analysis {
            all_vars.extend(var_set);
        }
        all_vars.len()
    }

    pub fn build(&self) -> InterferenceGraph {
        let mut graph = UnGraph::default();
        let number_of_variables_used = self.number_of_variables_used();
        let mut var_ix_to_node_ix = HashMap::new();
        for var_ix in 0..number_of_variables_used {
            let node_ix = graph.add_node(var_ix);
            var_ix_to_node_ix.insert(var_ix, node_ix);
        }

        for live_set in self.liveness_analysis {
            let live_set: Vec<_> = live_set
                .iter()
                .map(|var_ix| var_ix_to_node_ix[var_ix])
                .collect();
            for i in 0..live_set.len() {
                for j in i + 1..live_set.len() {
                    graph.add_edge(live_set[i], live_set[j], ());
                }
            }
        }
        InterferenceGraph::new(graph)
    }
}
