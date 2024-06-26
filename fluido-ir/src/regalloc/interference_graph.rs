#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use std::collections::{HashMap, HashSet};

use petgraph::prelude::UnGraph;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

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

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn dot(&self) -> String {
        format!("{:?}", petgraph::dot::Dot::new(&self.graph))
    }

    pub fn try_coloring(&self, number_of_colors: u64) -> Option<HashMap<usize, u64>> {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);
        let graph = &self.graph;

        let node_to_int: HashMap<_, _> = graph
            .node_indices()
            .map(|node_ix| graph[node_ix])
            .map(|k| (k, Int::new_const(&ctx, format!("t{}", k))))
            .collect();

        let zero = Int::from_u64(&ctx, 0);
        let num_colors = Int::from_u64(&ctx, number_of_colors);
        for node_ix in graph.node_indices() {
            let node = graph[node_ix];
            let var = &node_to_int[&node];
            solver.assert(&var.ge(&zero));
            solver.assert(&var.lt(&num_colors));
            for neighbor_ix in graph.neighbors(node_ix) {
                let neighbor_node = graph[neighbor_ix];
                let neighbor_var = &node_to_int[&neighbor_node];
                solver.assert(&var._eq(neighbor_var).not())
            }
        }

        match solver.check() {
            z3::SatResult::Unsat => {
                // TODO: add logging to enable dbg statements
                //println!("cannot color the graph with {} colors", number_of_colors);
                return None;
            }
            z3::SatResult::Unknown => {
                //println!("unknown returned from the z3 solver for graph coloring");
                return None;
            }
            z3::SatResult::Sat => {}
        }

        let model = solver
            .get_model()
            .expect("expected to get the model from solver");
        let mut node_to_color = HashMap::new();
        for node_ix in graph.node_indices() {
            let node = graph[node_ix];
            let node_var = &node_to_int[&node];
            let color_value = model
                .eval(node_var, true)
                .expect("expected to find a color value");
            let color_value = color_value
                .as_u64()
                .expect("expected to get color value as u64");
            node_to_color.insert(node, color_value);
        }

        Some(node_to_color)
    }

    /// Makes a binary search between 1 and max degree of the interference graph to find minimum
    /// number of colors needed to color the graph.
    pub fn find_min_color_count(&self) -> u64 {
        let graph = &self.graph;
        let max_degreee = graph
            .node_indices()
            .map(|node_ix| graph.edges(node_ix).count())
            .max()
            .expect("expected to find a maxdegree for the interference graph");
        let mut min_color_count = 1;
        let mut max_color_count = max_degreee + 1;
        let mut current_min = max_color_count;
        while min_color_count <= max_color_count {
            let color_count = (min_color_count + max_color_count) / 2;
            let result = self.try_coloring(color_count as u64);
            if result.is_some() {
                if color_count < current_min {
                    current_min = color_count;
                }
                max_color_count = color_count - 1;
            } else {
                min_color_count = color_count + 1;
            }
        }

        current_min as u64
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_graph_builder() {
        let liveness_analysis = vec![
            vec![0, 1].into_iter().collect(),
            vec![1, 2].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![3, 4].into_iter().collect(),
        ];
        let builder = InterferenceGraphBuilder::new(&liveness_analysis);
        let graph = builder.build();

        assert_eq!(graph.graph.node_count(), 5);
        assert_eq!(graph.graph.edge_count(), 4);
    }

    #[test]
    fn test_try_coloring_success() {
        let liveness_analysis = vec![
            vec![0, 1].into_iter().collect(),
            vec![1, 2].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![3, 4].into_iter().collect(),
        ];
        let builder = InterferenceGraphBuilder::new(&liveness_analysis);
        let graph = builder.build();

        let coloring = graph.try_coloring(5);
        assert!(coloring.is_some());
        let coloring = coloring.unwrap();
        assert_eq!(coloring.len(), 5);
    }

    #[test]
    fn test_try_coloring_failure() {
        let liveness_analysis = vec![
            vec![0, 1].into_iter().collect(),
            vec![1, 2].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![3, 4].into_iter().collect(),
        ];
        let builder = InterferenceGraphBuilder::new(&liveness_analysis);
        let graph = builder.build();

        let coloring = graph.try_coloring(1);
        assert!(coloring.is_none());
    }

    #[test]
    fn test_find_min_color_count() {
        let liveness_analysis = vec![
            vec![0, 1].into_iter().collect(),
            vec![1, 2].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![3, 4].into_iter().collect(),
        ];
        let builder = InterferenceGraphBuilder::new(&liveness_analysis);
        let graph = builder.build();

        let min_colors = graph.find_min_color_count();
        assert_eq!(min_colors, 2);
    }
}
