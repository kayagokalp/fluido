use std::collections::HashSet;

use crate::{
    ir::{IROp, Operand},
    pass_manager::{AnalysisPass, AnalysisResult},
};

#[derive(Default)]
pub struct LivenessAnalysis {}

impl AnalysisPass for LivenessAnalysis {
    fn analyze(&self, ir_to_pass_over: &[IROp]) -> crate::pass_manager::AnalysisResult {
        let mut live_regs = vec![];
        let mut ir = ir_to_pass_over.to_vec();
        ir.reverse();
        for (ix, op) in ir.iter().enumerate() {
            let mut live_set = if ix != 0 {
                live_regs.get(ix - 1).cloned().unwrap_or_else(HashSet::new)
            } else {
                HashSet::new()
            };
            let target = match op {
                IROp::Store(store_op) => &store_op.1,
                IROp::Mix(mix_op) => &mix_op.2,
            };
            let target_vreg = if let Operand::VirtualRegister(ix) = target {
                ix
            } else {
                panic!("expected v reg as operand for liveness analysis")
            };
            // remove current target vreg as it is now overriden so no longer live.
            live_set.retain(|elem| elem != target_vreg);

            let gen_set = match op {
                IROp::Store(_) => HashSet::new(),
                IROp::Mix(mix_op) => {
                    let vreg_1 = if let Operand::VirtualRegister(ix) = mix_op.0 {
                        ix
                    } else {
                        panic!("expected v reg as operand for liveness analysis")
                    };
                    let vreg_2 = if let Operand::VirtualRegister(ix) = mix_op.1 {
                        ix
                    } else {
                        panic!("expected v reg as operand for liveness analysis")
                    };

                    HashSet::from([vreg_1, vreg_2])
                }
            };
            live_set.extend(gen_set);
            live_regs.push(live_set);
        }
        live_regs.reverse();
        AnalysisResult {
            sets_per_ir: live_regs,
        }
    }

    fn pass_name(&self) -> &str {
        "liveness"
    }
}

#[cfg(test)]
mod tests {
    use super::LivenessAnalysis;
    use crate::{graph::Graph, ir::IROp, ir_builder::IRBuilder, pass_manager::AnalysisPass};
    use fluido_parse::parser::Parse;
    use fluido_types::expr::Expr;
    use std::collections::HashSet;

    fn ir_from_str(input_str: &str) -> Vec<IROp> {
        let mix_expr_parsed = Expr::parse(input_str).unwrap();
        let mixer_graph = Graph::from(&mix_expr_parsed);
        let mut ir_builder = IRBuilder::default();
        ir_builder.build_ir(mixer_graph)
    }

    #[test]
    fn single_mix_test() {
        let mix_expr = "(mix (fluid 0.2 1) (fluid 0.2 1))";
        let ir = ir_from_str(mix_expr);
        let liveness_analysis = LivenessAnalysis {};
        let result = liveness_analysis.analyze(&ir);

        let expected_sets = vec![HashSet::from([]), HashSet::from([0]), HashSet::from([0, 1])];
        let result_sets = result.sets_per_ir;

        assert_eq!(expected_sets, result_sets)
    }
}
