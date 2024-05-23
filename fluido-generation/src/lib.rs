use egg::{rewrite as rw, *};
use fluido_types::{
    concentration::{Concentration, LimitedFloat, Volume},
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

                let val_a = node_a.clone().expect_limited_float().unwrap();
                let val_b = node_b.clone().expect_limited_float().unwrap();
                let result = val_a + val_b;
                ArithmeticAnalysisPayload::LimitedFloat(result)
            }
            MixLang::Sub(sub) => {
                let node_a_id = sub[0];
                let node_b_id = sub[1];

                let node_a = &egraph[node_a_id].data;
                let node_b = &egraph[node_b_id].data;

                let val_a = node_a.clone().expect_limited_float().unwrap();
                let val_b = node_b.clone().expect_limited_float().unwrap();

                let result = val_a - val_b;
                ArithmeticAnalysisPayload::LimitedFloat(result)
            }
            MixLang::Div(div) => {
                let node_a_id = div[0];
                let node_b_id = div[1];

                let node_a = egraph[node_a_id].clone().data;
                let node_b = egraph[node_b_id].clone().data;

                let val_a = node_a.clone().expect_limited_float().unwrap();
                let val_b = node_b.clone().expect_limited_float().unwrap();
                let result = val_a.clone() / val_b.clone();
                ArithmeticAnalysisPayload::LimitedFloat(result)
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
            let concentration = fl.concentration();
            let concentration_node = egraph.add(MixLang::LimitedFloat(concentration.clone()));
            let volume = fl.unit_volume();
            let volume_node = egraph.add(MixLang::LimitedFloat(volume.clone()));
            let added = egraph.add(MixLang::Fluid([concentration_node, volume_node]));
            egraph.union(id, added);
        }
    }
}

pub struct OpCost<'a> {
    target: Concentration,
    input_space: HashSet<Concentration>,
    egraph: &'a EGraph<MixLang, ArithmeticAnalysis>,
}

impl<'a> OpCost<'a> {
    pub(crate) fn new(
        target: Concentration,
        input_space: HashSet<Concentration>,
        egraph: &'a EGraph<MixLang, ArithmeticAnalysis>,
    ) -> Self {
        Self {
            target,
            input_space,
            egraph,
        }
    }

    fn is_fluid_in_input_space(&self, fluid: &Fluid) -> bool {
        self.input_space.contains(&fluid.concentration())
    }

    fn is_direct_fluid_available(&self, fluid: &Fluid) -> bool {
        self.is_fluid_in_input_space(fluid)
    }

    fn proximity_cost(&self, conc: &Concentration) -> f64 {
        let mut min = 1.0;
        for val in self.input_space.iter() {
            let diff = conc.clone() - val.clone();
            let diff: f64 = diff.into();
            let diff = diff.abs();
            if diff < min {
                min = diff;
            }
        }
        min
    }
}

impl<'a> egg::CostFunction<MixLang> for OpCost<'a> {
    type Cost = f64;

    fn cost<C>(&mut self, enode: &MixLang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let base_cost = match enode {
            MixLang::LimitedFloat(_) => 0.0,
            MixLang::Add(_) => 100.0,
            MixLang::Sub(_) => 100.0,
            MixLang::Div(_) => 100.0,
            MixLang::Mix(_) => 0.0,
            MixLang::Fluid(fl) => {
                let conc_id = fl[0];
                let vol_id = fl[1];

                if let (Some(conc), Some(vol)) = (
                    self.egraph[conc_id].data.clone().expect_limited_float(),
                    self.egraph[vol_id].data.clone().expect_limited_float(),
                ) {
                    let fluid = Fluid::new(conc, vol);
                    let concentration = fluid.concentration();
                    if self.is_direct_fluid_available(&fluid) {
                        1.0
                    } else if self.target == *concentration {
                        100.0
                    } else {
                        self.proximity_cost(concentration)
                    }
                } else {
                    1000.0
                }
            }
        };
        enode.fold(base_cost, |sum, id| sum + costs(id))
    }
}

fn generate_rewrite_rules(
    input_space: HashSet<Concentration>,
) -> Vec<Rewrite<MixLang, ArithmeticAnalysis>> {
    vec![
        rw!("expand-fluid-to-mix";
            "(fluid ?a ?b)" => "(mix (fluid ?a (/ ?b 2.0)) (fluid ?a (/ ?b 2.0)))"
            if (fluid_valid("?a", "?b", input_space))),
        rw!("diff-mixers-l";
            "(mix (fluid ?a ?b) (fluid ?c ?b))" => "(mix (fluid (+ ?a 0.01) ?b) (fluid (- ?c 0.01) ?b))"
            if concentration_valid("?a", Op::Add, "?c", Op::Remove, 0.01)),
        rw!("diff-mixers-r";
            "(mix (fluid ?a ?b) (fluid ?c ?b))" => "(mix (fluid (- ?a 0.01) ?b) (fluid (+ ?c 0.01) ?b))"
            if concentration_valid("?a", Op::Remove, "?c", Op::Add, 0.01)),
    ]
}

fn fluid_valid(
    con: &'static str,
    vol: &'static str,
    input_space: HashSet<Concentration>,
) -> impl Fn(&mut EGraph<MixLang, ArithmeticAnalysis>, Id, &Subst) -> bool {
    let var_vol: Var = vol.parse().unwrap();
    let var_con: Var = con.parse().unwrap();
    move |egraph, _, subst| {
        let vol = subst[var_vol];
        let vol_node = &egraph[vol];
        let vol = vol_node.data.clone().expect_limited_float().unwrap();
        let two = Volume::from(2.0);
        let res = vol / two;
        let res: f64 = res.into();

        let col = subst[var_con];
        let col_node = &egraph[col];
        let col = col_node.data.clone().expect_limited_float().unwrap();
        input_space.contains(&col);

        res > 0.0 && res <= 1.0 && !input_space.contains(&col)
    }
}

enum Op {
    Add,
    Remove,
}

fn concentration_valid(
    concentration_a: &'static str,
    op_a: Op,
    concentration_b: &'static str,
    op_b: Op,

    step: f64,
) -> impl Fn(&mut EGraph<MixLang, ArithmeticAnalysis>, Id, &Subst) -> bool {
    let var_concentration_a: Var = concentration_a.parse().unwrap();
    let var_concentration_b: Var = concentration_b.parse().unwrap();
    move |egraph, _, subst| {
        let conc_a = subst[var_concentration_a];
        let conc_node_a = &egraph[conc_a];
        let concentration_a = conc_node_a.data.clone().expect_limited_float().unwrap();
        let concentration_a: f64 = concentration_a.into();
        let res = match op_a {
            Op::Add => concentration_a + step,
            Op::Remove => concentration_a - step,
        };
        let concentration_a = Concentration::from(res);

        let conc_b = subst[var_concentration_b];
        let conc_node_b = &egraph[conc_b];
        let concentration_b = conc_node_b.data.clone().expect_limited_float().unwrap();
        let concentration_b: f64 = concentration_b.into();
        let res = match op_b {
            Op::Add => concentration_b + step,
            Op::Remove => concentration_b - step,
        };
        let concentration_b = Concentration::from(res);

        concentration_a.valid() && concentration_b.valid()
    }
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
    let target_node = format!("(fluid {} 1.0)", target_concentration)
        .parse::<RecExpr<MixLang>>()
        .map_err(|_| MixerGenerationError::FailedToParseTarget(target_concentration.clone()))?;

    let target = initial_egraph.add_expr(&target_node);
    println!("{target_node:?}, target {target_concentration:?}");

    /*
        for fluid in generate_all_fluids().iter().filter_map(|fl| {
            format!("(fluid {} {})", fl.concentration(), fl.unit_volume())
                .parse::<RecExpr<MixLang>>()
                .ok()
        }) {
            initial_egraph.add_expr(&fluid);
        }
    */
    let input_space = input_space
        .iter()
        .map(|fluid| fluid.concentration())
        .cloned()
        .collect::<HashSet<_>>();

    let runner: Runner<MixLang, ArithmeticAnalysis, ()> = Runner::new(ArithmeticAnalysis)
        .with_egraph(initial_egraph)
        .with_node_limit(10000000000000000)
        .with_iter_limit(100000)
        .with_time_limit(Duration::from_secs(time_limit))
        .run(&generate_rewrite_rules(input_space.clone()));

    runner.print_report();

    let extractor = Extractor::new(
        &runner.egraph,
        OpCost::new(target_concentration, input_space, &runner.egraph),
    );

    let (cost, best_expr) = extractor.find_best(target);
    println!("{best_expr} cost {cost}");
    let sequence = Sequence { cost, best_expr };
    Ok(sequence)
}

pub struct Sequence {
    pub cost: f64,
    pub best_expr: RecExpr<MixLang>,
}
