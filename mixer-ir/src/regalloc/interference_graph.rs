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
            z3::SatResult::Sat => println!("graph can be colored with {} colors", number_of_colors),
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
