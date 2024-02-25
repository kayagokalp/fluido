use crate::ir::IROp;
use std::collections::{HashMap, HashSet};

/// Manages possible analysis passes over flat mixlang ir.
pub struct IRPassManager<'a> {
    ir_to_pass_over: Vec<IROp>,
    analysis_passes: Vec<&'a dyn AnalysisPass>,
}

impl<'a> IRPassManager<'a> {
    pub fn new(ir_to_pass_over: Vec<IROp>, analysis_passes: Vec<&'a dyn AnalysisPass>) -> Self {
        Self {
            ir_to_pass_over,
            analysis_passes,
        }
    }

    pub fn register_analysis_pass(&mut self, pass_to_register: &'a dyn AnalysisPass) {
        self.analysis_passes.push(pass_to_register);
    }

    /// Returns results of registered analysis passes. In the form of `(pass_name, analysis result)`.
    pub fn apply_analysis_passes(&self) -> HashMap<&str, AnalysisResult> {
        let ir_to_pass_over = self.ir_to_pass_over.as_slice();
        self.analysis_passes
            .iter()
            .map(|analysis_pass| {
                (
                    analysis_pass.pass_name(),
                    analysis_pass.analyze(ir_to_pass_over),
                )
            })
            .collect()
    }
}

#[derive(Default, Debug)]
pub struct AnalysisResult {
    pub sets_per_ir: Vec<HashSet<usize>>,
}

pub trait AnalysisPass {
    fn pass_name(&self) -> &str;
    fn analyze(&self, ir_to_pass_over: &[IROp]) -> AnalysisResult;
}
