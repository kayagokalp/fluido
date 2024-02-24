use crate::ir::IROp;
use std::collections::HashSet;

/// Manages possible analysis passes over flat mixlang ir.
#[derive(Default)]
pub struct IRPassManager<'a> {
    ir_to_pass_over: Vec<IROp>,
    analysis_passes: Vec<&'a dyn AnalysisPass>,
}

#[derive(Default, Debug)]
pub struct AnalysisResult {
    pub sets_per_ir: Vec<HashSet<usize>>,
}

pub trait AnalysisPass {
    fn analyze(&self, ir_to_pass_over: Vec<IROp>) -> AnalysisResult;
}
