use crate::parse::Expr;
use petgraph::graph::{DiGraph, NodeIndex};

pub struct Graph {
    graph: DiGraph<Expr, ()>,
}

impl Graph {
    fn new() -> Self {
        Self {
            graph: DiGraph::new(),
        }
    }

    fn add_expr(&mut self, expr: &Expr) -> NodeIndex {
        let index = self.graph.add_node(expr.clone());

        match expr {
            Expr::Number(_) => {}
            Expr::Mix(left, right) => {
                let left_index = self.add_expr(left);
                let right_index = self.add_expr(right);
                self.graph.add_edge(index, left_index, ());
                self.graph.add_edge(index, right_index, ());
            }
        }
        index
    }

    pub fn dot(&self) -> String {
        format!(
            "{:?}",
            petgraph::dot::Dot::with_attr_getters(
                &self.graph,
                &[
                    petgraph::dot::Config::NodeNoLabel,
                    petgraph::dot::Config::EdgeNoLabel
                ],
                &|_, er| format!("label = \"{:?}\"", er.weight()),
                &|_, nr| {
                    let _node = &self.graph[nr.0];
                    let node_label = match _node {
                        Expr::Mix(_, _) => "mix".to_string(),
                        Expr::Number(con) => format!("{}", con),
                    };
                    format!("label = {}", node_label)
                },
            )
        )
    }
}

impl From<&Expr> for Graph {
    fn from(expr: &Expr) -> Self {
        let mut wrapper = Graph::new();
        wrapper.add_expr(expr);
        wrapper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        let expr_str = "0.5";
        let expr: Expr = expr_str.parse().expect("Failed to parse expression");
        let graph_wrapper: Graph = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 1);
        assert_eq!(graph_wrapper.graph.edge_count(), 0);
    }

    #[test]
    fn test_simple_mix() {
        let expr_str = "(mix 0.1 0.2)";
        let expr: Expr = expr_str.parse().expect("Failed to parse expression");
        let graph_wrapper: Graph = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 3); // One Mix and two Numbers
        assert_eq!(graph_wrapper.graph.edge_count(), 2); // Two edges from Mix to Numbers
    }

    #[test]
    fn test_nested_mix() {
        let expr_str = "(mix(mix 0.0 0.2) 0.1)";
        let expr: Expr = expr_str.parse().expect("Failed to parse expression");
        let graph_wrapper: Graph = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 5); // Two Mix and three Numbers
        assert_eq!(graph_wrapper.graph.edge_count(), 4); // Edges connecting Mixes to Numbers
    }
}
