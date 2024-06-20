#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use fluido_types::number::SaturationNumber;

use crate::ir::IROp;
use std::collections::{HashMap, HashSet};

/// Manages possible analysis passes over flat mixlang ir.
pub struct IRPassManager<'a, T: SaturationNumber> {
    ir_to_pass_over: Vec<IROp<T>>,
    analysis_passes: Vec<&'a dyn AnalysisPass<T>>,
}

impl<'a, T: SaturationNumber> IRPassManager<'a, T> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn new(
        ir_to_pass_over: Vec<IROp<T>>,
        analysis_passes: Vec<&'a dyn AnalysisPass<T>>,
    ) -> Self {
        Self {
            ir_to_pass_over,
            analysis_passes,
        }
    }

    pub fn register_analysis_pass(&mut self, pass_to_register: &'a dyn AnalysisPass<T>) {
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

pub trait AnalysisPass<T: SaturationNumber> {
    fn pass_name(&self) -> &str;
    fn analyze(&self, ir_to_pass_over: &[IROp<T>]) -> AnalysisResult;
}

#[cfg(test)]
mod tests {
    use fluido_types::fluid::LimitedFloat;

    use super::{AnalysisPass, AnalysisResult, IRPassManager};
    use crate::ir::IROp;
    use std::collections::HashSet;

    #[derive(Debug)]
    struct DummyAnalysisPass {
        name: &'static str,
    }

    impl AnalysisPass<LimitedFloat> for DummyAnalysisPass {
        fn pass_name(&self) -> &str {
            self.name
        }

        fn analyze(&self, _ir_to_pass_over: &[IROp<LimitedFloat>]) -> AnalysisResult {
            AnalysisResult {
                sets_per_ir: vec![HashSet::new()],
            }
        }
    }

    #[test]
    fn test_register_analysis_pass() {
        let ir = vec![];
        let mut manager = IRPassManager::new(ir, vec![]);
        let pass = DummyAnalysisPass { name: "dummy_pass" };

        manager.register_analysis_pass(&pass);

        assert_eq!(manager.analysis_passes.len(), 1);
        assert_eq!(manager.analysis_passes[0].pass_name(), "dummy_pass");
    }

    #[test]
    fn test_apply_analysis_passes() {
        let ir = vec![];
        let pass1 = DummyAnalysisPass { name: "pass1" };
        let pass2 = DummyAnalysisPass { name: "pass2" };

        let mut manager = IRPassManager::new(ir, vec![&pass1]);
        manager.register_analysis_pass(&pass2);

        let results = manager.apply_analysis_passes();

        assert_eq!(results.len(), 2);
        assert!(results.contains_key("pass1"));
        assert!(results.contains_key("pass2"));
        assert_eq!(results["pass1"].sets_per_ir.len(), 1);
        assert_eq!(results["pass2"].sets_per_ir.len(), 1);
    }

    #[test]
    fn test_analysis_result_default() {
        let result = AnalysisResult::default();
        assert!(result.sets_per_ir.is_empty());
    }

    #[test]
    fn test_dummy_analysis_pass() {
        let ir = vec![];
        let pass = DummyAnalysisPass { name: "dummy_pass" };

        let result = pass.analyze(&ir);

        assert_eq!(pass.pass_name(), "dummy_pass");
        assert_eq!(result.sets_per_ir.len(), 1);
        assert!(result.sets_per_ir[0].is_empty());
    }
}
