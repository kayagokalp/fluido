use fluido_types::{expr::Expr, number::SaturationNumber};
use petgraph::graph::{DiGraph, NodeIndex};

pub struct Graph<T: SaturationNumber> {
    graph: DiGraph<Expr<T>, ()>,
    root: Option<NodeIndex>,
}

impl<T: SaturationNumber> AsRef<DiGraph<Expr<T>, ()>> for Graph<T> {
    fn as_ref(&self) -> &DiGraph<Expr<T>, ()> {
        &self.graph
    }
}

impl<T: SaturationNumber> Graph<T> {
    fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            root: None,
        }
    }

    fn add_expr(&mut self, expr: &Expr<T>) -> NodeIndex {
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
                        Expr::Number(nm) => format!("{}", nm),
                    };
                    format!("label = {}", node_label)
                },
            )
        )
    }
}

impl<T: SaturationNumber> From<&Expr<T>> for Graph<T> {
    fn from(expr: &Expr<T>) -> Self {
        let mut wrapper = Graph::new();
        wrapper.add_expr(expr);
        wrapper
    }
}

#[cfg(test)]
mod tests {
    use fluido_parse::parser::Parse;
    use fluido_types::fluid::LimitedFloat;

    use super::*;

    #[test]
    fn test_single_fluid_lf() {
        let expr_str = "(fluid 0.5 1)";
        let expr = Expr::parse(expr_str).unwrap();
        let graph_wrapper: Graph<LimitedFloat> = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 1);
        assert_eq!(graph_wrapper.graph.edge_count(), 0);
    }

    #[test]
    fn test_simple_mix_lf() {
        let expr_str = "(mix (fluid 0.1 1) (fluid 0.2 1))";
        let expr = Expr::parse(expr_str).unwrap();
        let graph_wrapper: Graph<LimitedFloat> = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 3); // One Mix and two Numbers
        assert_eq!(graph_wrapper.graph.edge_count(), 2); // Two edges from Mix to Numbers
    }

    #[test]
    fn test_nested_mix_lf() {
        let expr_str = "(mix (mix (fluid 0.0 1) (fluid 0.2 1)) (fluid 0.1 1))";
        let expr = Expr::parse(expr_str).unwrap();
        let graph_wrapper: Graph<LimitedFloat> = (&expr).into();

        assert_eq!(graph_wrapper.graph.node_count(), 5); // Two Mix and three Numbers
        assert_eq!(graph_wrapper.graph.edge_count(), 4); // Edges connecting Mixes to Numbers
    }

    #[test]
    fn graph_to_dot_lf() {
        let expr_str = "(mix (mix (fluid 0.0 1) (fluid 0.2 1)) (fluid 0.1 1))";
        let expr = Expr::parse(expr_str).unwrap();
        let graph_wrapper: Graph<LimitedFloat> = (&expr).into();
        let dot = graph_wrapper.dot();
        let expected = "digraph {\n    0 [ label = mix]\n    1 [ label = mix]\n    2 [ label = (fluid 0.0 1.0)]\n    3 [ label = (fluid 0.2 1.0)]\n    4 [ label = (fluid 0.1 1.0)]\n    1 -> 2 [ label = \"()\"]\n    1 -> 3 [ label = \"()\"]\n    0 -> 1 [ label = \"()\"]\n    0 -> 4 [ label = \"()\"]\n}\n";
        assert_eq!(dot, expected)
    }
}
