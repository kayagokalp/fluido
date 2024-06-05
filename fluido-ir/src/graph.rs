use fluido_types::expr::Expr;
use petgraph::graph::{DiGraph, NodeIndex};

pub struct Graph {
    graph: DiGraph<Expr, ()>,
    root: Option<NodeIndex>,
}

impl AsRef<DiGraph<Expr, ()>> for Graph {
    fn as_ref(&self) -> &DiGraph<Expr, ()> {
        &self.graph
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            root: None,
        }
    }

    fn add_expr(&mut self, expr: &Expr) -> NodeIndex {
        let index = self.graph.add_node(expr.clone());
        if self.root.is_none() {
            self.root = Some(index);
        }

        if let Expr::Mix(left, right) = expr {
            let left_index = self.add_expr(left);
            let right_index = self.add_expr(right);
            self.graph.add_edge(index, left_index, ());
            self.graph.add_edge(index, right_index, ());
        }
        index
    }

    pub fn root_node(&self) -> Option<NodeIndex> {
        self.root
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
                        Expr::Fluid(fl) => format!("{}", fl),
                        Expr::LimitedFloat(fl) => format!("{}", fl),
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
    use fluido_parse::parser::Parse;

    use super::*;

    #[test]
    fn test_single_fluid() {
        let expr_str = "(fluid 0.5 1)";
        let expr = Expr::parse(expr_str).unwrap();
        let graph_wrapper: Graph = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 1);
        assert_eq!(graph_wrapper.graph.edge_count(), 0);
    }

    #[test]
    fn test_simple_mix() {
        let expr_str = "(mix (fluid 0.1 1) (fluid 0.2 1))";
        let expr = Expr::parse(expr_str).unwrap();
        let graph_wrapper: Graph = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 3); // One Mix and two Numbers
        assert_eq!(graph_wrapper.graph.edge_count(), 2); // Two edges from Mix to Numbers
    }

    #[test]
    fn test_nested_mix() {
        let expr_str = "(mix (mix (fluid 0.0 1) (fluid 0.2 1)) (fluid 0.1 1))";
        let expr = Expr::parse(expr_str).unwrap();
        let graph_wrapper: Graph = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 5); // Two Mix and three Numbers
        assert_eq!(graph_wrapper.graph.edge_count(), 4); // Edges connecting Mixes to Numbers
    }
}
