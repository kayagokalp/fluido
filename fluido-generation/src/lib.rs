use egg::{rewrite as rw, *};
use fluido_types::{
    concentration::{Concentration, LimitedFloat},
    error::MixerGenerationError,
    fluid::Fluid,
};
use std::{collections::HashSet, time::Duration};

define_language! {
    pub enum MixLang {
        LimitedFloat(LimitedFloat),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "/" = Div([Id; 2]),
        "mix" = Mix([Id; 2]),
        "fluid" = Fluid([Id; 2]),
    }
}
#[derive(Default)]
struct ArithmeticAnalysis;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ArithmeticAnalysisPayload {
    LimitedFloat(LimitedFloat),
    Fluid(Fluid),
    None,
}

impl ArithmeticAnalysisPayload {
    pub fn expect_limited_float(self) -> Option<LimitedFloat> {
        match self {
            ArithmeticAnalysisPayload::LimitedFloat(lf) => Some(lf),
            _ => None,
        }
    }
}

impl MixLang {
    // TODO: rename this.
    pub fn expect_limited_float(self) -> Option<LimitedFloat> {
        match self {
            MixLang::LimitedFloat(lf) => Some(lf),
            _ => None,
        }
    }
}

impl Analysis<MixLang> for ArithmeticAnalysis {
    type Data = ArithmeticAnalysisPayload;

    fn make(egraph: &EGraph<MixLang, Self>, enode: &MixLang) -> Self::Data {
        match enode {
            MixLang::Mix(mix) => {
                let fluid_a = &egraph[mix[0]];
                let fluid_b = &egraph[mix[1]];

                let fluid_a_conc_id = &fluid_a.nodes[0].children()[0];
                let fluid_a_vol_id = &fluid_a.nodes[0].children()[1];
                let fluid_a_conc = &egraph[*fluid_a_conc_id].nodes[0]
                    .clone()
                    .expect_limited_float();
                let fluid_a_vol = &egraph[*fluid_a_vol_id].nodes[0]
                    .clone()
                    .expect_limited_float();

                let fluid_b_conc_id = &fluid_b.nodes[0].children()[0];
                let fluid_b_vol_id = &fluid_b.nodes[0].children()[1];
                let fluid_b_conc = &egraph[*fluid_b_conc_id].nodes[0]
                    .clone()
                    .expect_limited_float();
                let fluid_b_vol = &egraph[*fluid_b_vol_id].nodes[0]
                    .clone()
                    .expect_limited_float();

                if let (
                    Some(fluid_a_conc),
                    Some(fluid_a_vol),
                    Some(fluid_b_conc),
                    Some(fluid_b_vol),
                ) = (fluid_a_conc, fluid_a_vol, fluid_b_conc, fluid_b_vol)
                {
                    let fluid_a = Fluid::new(fluid_a_conc.clone(), fluid_a_vol.clone());
                    let fluid_b = Fluid::new(fluid_b_conc.clone(), fluid_b_vol.clone());

                    let mixed_fluid = fluid_a.mix(&fluid_b);
                    ArithmeticAnalysisPayload::Fluid(mixed_fluid)
                } else {
                    ArithmeticAnalysisPayload::None
                }
            }
            MixLang::Fluid(fl) => {
                let node_a_id = fl[0];
                let node_b_id = fl[1];

                let node_a = &egraph[node_a_id].data.clone().expect_limited_float();
                let node_b = &egraph[node_b_id].data.clone().expect_limited_float();

                if let (Some(conc), Some(vol)) = (node_a, node_b) {
                    let fl = Fluid::new(conc.clone(), vol.clone());
                    ArithmeticAnalysisPayload::Fluid(fl)
                } else {
                    ArithmeticAnalysisPayload::None
                }
            }
            MixLang::LimitedFloat(fl) => ArithmeticAnalysisPayload::LimitedFloat(fl.clone()),
            MixLang::Add(add) => {
                let node_a_id = add[0];
                let node_b_id = add[1];

                let node_a = &egraph[node_a_id].data;
                let node_b = &egraph[node_b_id].data;

                if let (Some(vol_a), Some(vol_b)) = (
                    node_a.clone().expect_limited_float(),
                    node_b.clone().expect_limited_float(),
                ) {
                    let result = vol_a + vol_b;
                    ArithmeticAnalysisPayload::LimitedFloat(result)
                } else {
                    ArithmeticAnalysisPayload::None
                }
            }
            MixLang::Sub(sub) => {
                let node_a_id = sub[0];
                let node_b_id = sub[1];

                let node_a = &egraph[node_a_id].data;
                let node_b = &egraph[node_b_id].data;

                if let (Some(vol_a), Some(vol_b)) = (
                    node_a.clone().expect_limited_float(),
                    node_b.clone().expect_limited_float(),
                ) {
                    let result = vol_a - vol_b;
                    ArithmeticAnalysisPayload::LimitedFloat(result)
                } else {
                    ArithmeticAnalysisPayload::None
                }
            }
            MixLang::Div(div) => {
                let node_a_id = div[0];
                let node_b_id = div[1];

                let node_a = egraph[node_a_id].clone().data;
                let node_b = egraph[node_b_id].clone().data;
                if let (Some(vol_a), Some(vol_b)) = (
                    node_a.clone().expect_limited_float(),
                    node_b.clone().expect_limited_float(),
                ) {
                    let result = vol_a / vol_b;
                    ArithmeticAnalysisPayload::LimitedFloat(result)
                } else {
                    ArithmeticAnalysisPayload::None
                }
            }
        }
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        let mut to = match to {
            ArithmeticAnalysisPayload::None => None,
            a => Some(a),
        };
        let mut binding = match from {
            ArithmeticAnalysisPayload::None => None,
            a => Some(a),
        };
        let from = binding.as_mut();

        merge_option(&mut to, from, |a, b| {
            assert_eq!(*a, b, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph<MixLang, Self>, id: Id) {
        if let ArithmeticAnalysisPayload::Fluid(fl) = egraph[id].data.clone() {
            if fl.unit_volume().valid() && fl.unit_volume().wrapped != 0 {
                println!("adding (fluid {} {})", fl.concentration(), fl.unit_volume());
            }
            let concentration = fl.concentration();
            let concentration_node = egraph.add(MixLang::LimitedFloat(concentration.clone()));
            let volume = fl.unit_volume();
            let volume_node = egraph.add(MixLang::LimitedFloat(volume.clone()));
            let added = egraph.add(MixLang::Fluid([concentration_node, volume_node]));
            egraph.union(id, added);
        }
    }
}

struct SillyCostFn<'a> {
    input_space: HashSet<Concentration>,
    target: Concentration,
    egraph: &'a EGraph<MixLang, ArithmeticAnalysis>,
}

impl<'a> SillyCostFn<'a> {
    fn new(
        input_space: HashSet<Concentration>,
        target: Concentration,
        egraph: &'a EGraph<MixLang, ArithmeticAnalysis>,
    ) -> Self {
        Self {
            input_space,
            target,
            egraph,
        }
    }
}
impl CostFunction<MixLang> for SillyCostFn<'_> {
    type Cost = f64;

    fn cost<C>(&mut self, enode: &MixLang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let op_cost = match enode {
            MixLang::Mix(_) => 0.0,
            MixLang::Fluid(fl) => {
                let fl_node_a_id = fl[0];
                let fl_node_b_id = fl[1];

                let fl_node_a = &self.egraph[fl_node_a_id].data;
                let fl_node_b = &self.egraph[fl_node_b_id].data;

                let fl_conc = fl_node_a.clone().expect_limited_float();
                let fl_vol = fl_node_b.clone().expect_limited_float();

                if let (Some(conc), Some(vol)) = (fl_conc, fl_vol) {
                    if conc == self.target {
                        10000.0
                    } else if self.input_space.contains(&conc) {
                        0.0
                    } else {
                        let closest_val = self
                            .input_space
                            .iter()
                            .map(|input| (input.clone() - conc.clone()).wrapped.abs())
                            .min();

                        let final_val = closest_val.unwrap() as f64 * LimitedFloat::EPSILON;
                        final_val
                    }
                } else {
                    f64::MAX
                }
            }
            _ => 100.0,
        };

        enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}

fn generate_rewrite_rules() -> Vec<Rewrite<MixLang, ArithmeticAnalysis>> {
    vec![
        rw!("expand-fluid-to-mix";
            "(fluid ?a ?b)" => "(mix (fluid ?a (/ ?b 2)) (fluid ?a (/ ?b 2)))"),
        rw!("diff-mixers-l";
            "(mix (fluid ?a ?b) (fluid ?c ?b))" => "(mix (fluid (+ ?a 0.01) ?b) (fluid (- ?c 0.01) ?b))"),
        rw!("diff-mixers-r";
            "(mix (fluid ?a ?b) (fluid ?c ?b))" => "(mix (fluid (- ?a 0.01) ?b) (fluid (+ ?c 0.01) ?b))"),
    ]
}

/// Generate all possible fluids with given start and end with step sizes.
pub fn generate_all_fluids() -> Vec<Fluid> {
    let epsilon = Concentration::EPSILON;
    let end = (1.0 / epsilon) as usize;

    let mut result = Vec::with_capacity(end as usize);
    for i in 0..end {
        let concentrationtion = Concentration::new(i as i64);
        let volume = 1.0.into();
        let fluid = Fluid::new(concentrationtion, volume);
        result.push(fluid);
    }

    result
}

/// Saturate to find out an optimized sequence according to the cost function.
pub fn saturate(
    target_concentration: Concentration,
    time_limit: u64,
    input_space: &[Fluid],
) -> Result<Sequence, MixerGenerationError> {
    let mut initial_egraph = EGraph::new(ArithmeticAnalysis);
    let start_node = format!("(fluid {} 1)", target_concentration)
        .parse::<RecExpr<MixLang>>()
        .map_err(|_| MixerGenerationError::FailedToParseTarget(target_concentration.clone()))?;
    let target = initial_egraph.add_expr(&start_node);
    println!("{start_node:?}, target {target_concentration:?}");
    for fluid in generate_all_fluids().iter().filter_map(|fl| {
        format!("(fluid {} {})", fl.concentration(), fl.unit_volume())
            .parse::<RecExpr<MixLang>>()
            .ok()
    }) {
        initial_egraph.add_expr(&fluid);
    }

    let runner: Runner<MixLang, ArithmeticAnalysis, ()> = Runner::new(ArithmeticAnalysis)
        .with_egraph(initial_egraph)
        .with_node_limit(10000000000000000)
        .with_iter_limit(100000)
        .with_time_limit(Duration::from_secs(time_limit))
        .run(&generate_rewrite_rules());

    runner.print_report();

    let input_space = input_space
        .iter()
        .map(|fluid| fluid.concentration())
        .cloned()
        .collect::<HashSet<_>>();

    let extractor = Extractor::new(
        &runner.egraph,
        SillyCostFn::new(input_space, target_concentration, &runner.egraph),
    );

    println!("{:?}", &runner.egraph[target].data);
    let (cost, best_expr) = extractor.find_best(target);
    println!("{best_expr} cost: {cost}");
    let sequence = Sequence { cost, best_expr };
    Ok(sequence)
}

pub struct Sequence {
    pub cost: f64,
    pub best_expr: RecExpr<MixLang>,
}
