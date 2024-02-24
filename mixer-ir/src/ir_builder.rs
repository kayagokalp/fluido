use crate::ir::{IROp, Operand};
use mixer_generator::concentration::Concentration;
use mixer_graph::{graph::Graph, parse::Expr};

#[derive(Debug, Default)]
pub struct IRBuilder {
    context: IRContext,
}

#[derive(Debug, Default)]
pub struct IRContext {
    ir_output: Vec<IROp>,
}

impl IRBuilder {
    pub fn build_ir(&mut self, graph: Graph) -> Vec<IROp> {
        let root_node = graph.root_node().expect("missing root node in graph");
        let expr = &graph.as_ref()[root_node];
        self.compile_expr(expr.clone());
        self.context.ir_output.clone()
    }

    /// Returns the expr's result v_reg.
    pub fn compile_expr(&mut self, expr: Expr) -> usize {
        match expr {
            Expr::Mix(l_expr, r_expr) => self.compile_mix(*l_expr, *r_expr),
            Expr::Number(concentration) => self.compile_number(concentration),
        }
    }

    pub fn compile_number(&mut self, number: Concentration) -> usize {
        let current_virtual_register_ix = self.context.ir_output.len();
        let store_destination_v_reg = Operand::VirtualRegister(current_virtual_register_ix);
        let value_to_store = Operand::Const(number);
        let ir_op = IROp::Store((value_to_store, store_destination_v_reg));
        self.context.ir_output.push(ir_op);
        current_virtual_register_ix
    }

    pub fn compile_mix(&mut self, lhs: Expr, rhs: Expr) -> usize {
        let lhs_vreg_ix = self.compile_expr(lhs);
        let rhs_vreg_ix = self.compile_expr(rhs);
        let current_virtual_register_ix = self.context.ir_output.len();
        let lhs_vreg_operand = Operand::VirtualRegister(lhs_vreg_ix);
        let rhs_vreg_operand = Operand::VirtualRegister(rhs_vreg_ix);
        let target_vreg = Operand::VirtualRegister(current_virtual_register_ix);

        let ir_op = IROp::Mix((lhs_vreg_operand, rhs_vreg_operand, target_vreg));

        self.context.ir_output.push(ir_op);
        current_virtual_register_ix
    }
}
