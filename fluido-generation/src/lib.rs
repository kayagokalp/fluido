use egg::{rewrite as rw, *};
use fluido_types::{concentration::Concentration, error::MixerGenerationError, fluid::Fluid};
use std::{collections::HashSet, time::Duration};

define_language! {
    pub enum MixLang {
        Vol(u64),
        Concentration(Concentration),
        "mix" = Mix([Id; 2]),
        "fluid" = Fluid([Id; 2]),
        "c+" = ConcentrationAdd([Id; 2]),
        "c-" = ConcentrationSub([Id; 2]),
    }
}
#[derive(Default)]
struct ArithmeticAnalysis;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ArithmeticAnalysisPayload {
    Vol(u64),
    Concentration(Concentration),
    Fluid(Fluid),
    None,
}

impl ArithmeticAnalysisPayload {
    fn expect_vol(self) -> u64 {
        match self {
            ArithmeticAnalysisPayload::Vol(vol) => vol,
            a => panic!("tried to get vol from payload, got {a:?}"),
        }
    }

    fn expect_concentration(self) -> Concentration {
        match self {
            ArithmeticAnalysisPayload::Concentration(conc) => conc,
            a => panic!("tried to get concentration from payload, got {a:?}"),
        }
    }

    fn expect_fluid(self) -> Fluid {
        match self {
            ArithmeticAnalysisPayload::Fluid(fluid) => fluid,
            a => panic!("tried to get fluid from payload, got {a:?}"),
        }
    }
}

impl Analysis<MixLang> for ArithmeticAnalysis {
    type Data = ArithmeticAnalysisPayload;

    fn make(egraph: &EGraph<MixLang, Self>, enode: &MixLang) -> Self::Data {
        match enode {
            MixLang::Mix(_) => ArithmeticAnalysisPayload::None,
            MixLang::Fluid(fluid) => {
                let conc = fluid[0];
                let vol = fluid[1];

                let conc_node = &egraph[conc];
                let vol_node = &egraph[vol];

                let concentration = conc_node.data.clone().expect_concentration();
                let vol = vol_node.data.clone().expect_vol();

                let fluid = Fluid::new(concentration, vol);

                ArithmeticAnalysisPayload::Fluid(fluid)
            }
            MixLang::ConcentrationAdd(add) => {
                let node_a = add[0];
                let node_b = add[1];

                let node_a_fluid = egraph[node_a].data.clone().expect_fluid();
                let node_b_fluid = egraph[node_b].data.clone().expect_fluid();

                let node_a_unit_vol = node_a_fluid.unit_volume();
                let node_b_unit_vol = node_b_fluid.unit_volume();

                // SAFETY: This is for sanity check, for `ConcentrationAdd` to be applied to any
                // term during saturation, we require the volumes to be equal. So this should not
                // fail. If it is failing, this is a critical bug.
                assert_eq!(node_a_unit_vol, node_b_unit_vol);

                let node_a_concentration = node_a_fluid.concentration();
                let node_b_concentration = node_b_fluid.concentration();

                let concentration = node_a_concentration.clone() + node_b_concentration.clone();
                let new_fluid = Fluid::new(concentration, node_a_unit_vol);
                ArithmeticAnalysisPayload::Fluid(new_fluid)
            }
            MixLang::ConcentrationSub(sub) => {
                let node_a = sub[0];
                let node_b = sub[1];

                let node_a_fluid = egraph[node_a].data.clone().expect_fluid();
                let node_b_fluid = egraph[node_b].data.clone().expect_fluid();

                let node_a_unit_vol = node_a_fluid.unit_volume();
                let node_b_unit_vol = node_b_fluid.unit_volume();

                // SAFETY: This is for sanity check, for `ConcentrationAdd` to be applied to any
                // term during saturation, we require the volumes to be equal. So this should not
                // fail. If it is failing, this is a critical bug.
                assert_eq!(node_a_unit_vol, node_b_unit_vol);

                let node_a_concentration = node_a_fluid.concentration();
                let node_b_concentration = node_b_fluid.concentration();

                let concentration = node_a_concentration.clone() - node_b_concentration.clone();
                let new_fluid = Fluid::new(concentration, node_a_unit_vol);
                ArithmeticAnalysisPayload::Fluid(new_fluid)
            }
            MixLang::Vol(vol) => ArithmeticAnalysisPayload::Vol(*vol),
            MixLang::Concentration(conc) => ArithmeticAnalysisPayload::Concentration(conc.clone()),
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
            let volume = fl.unit_volume();
            let volume_node = egraph.add(MixLang::Vol(volume));

            let concentration = fl.concentration();
            let concentration_node = egraph.add(MixLang::Concentration(concentration.clone()));
            let added = egraph.add(MixLang::Fluid([concentration_node, volume_node]));
            egraph.union(id, added);
        }
    }
}

fn generate_mix_rules() -> Vec<Rewrite<MixLang, ArithmeticAnalysis>> {
    vec![
        rw!("differentiate-mixer-conc1";
            "(mix (fluid ?a ?c) (fluid ?b ?c))" => "(mix (c- (fluid ?a ?c) (fluid 0.001 ?c)) (c+ (fluid ?b ?c) (fluid 0.001 ?c)))"),
        rw!("differentiate-mixer-conc2";
            "(mix (fluid ?a ?c) (fluid ?b ?c))" => "(mix (c+ (fluid ?a ?c) (fluid 0.001 ?c)) (c- (fluid ?b ?c) (fluid 0.001 ?c)))"),
        rw!("expand-fluid";
            "(fluid ?a ?b)" => "(mix (fluid ?a ?b) (fluid ?a ?b))"),
    ]
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
            MixLang::Fluid(fluid) => {
                let concentration_id = fluid[0];
                let concentration = self.egraph[concentration_id]
                    .data
                    .clone()
                    .expect_concentration();
                if concentration == self.target {
                    f64::MAX
                } else if self.input_space.contains(&concentration) {
                    0.0
                } else {
                    let closest_val = self
                        .input_space
                        .iter()
                        .map(|input| (input.clone() - concentration.clone()).wrapped.abs())
                        .min();

                    closest_val.unwrap() as f64
                }
            }
            _ => 100.0,
        };

        enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}

/// Saturate to find out an optimized sequence according to the cost function.
pub fn saturate(
    target_concentration: Concentration,
    time_limit: u64,
    input_space: &[Concentration],
) -> Result<Sequence, MixerGenerationError> {
    let start = format!("(fluid {} 1)", target_concentration)
        .parse()
        .map_err(|_| MixerGenerationError::FailedToParseTarget(target_concentration.clone()))?;
    let runner: Runner<MixLang, ArithmeticAnalysis, ()> = Runner::new(ArithmeticAnalysis)
        .with_expr(&start)
        .with_node_limit(10000000000000000)
        .with_iter_limit(100000)
        .with_time_limit(Duration::from_secs(time_limit))
        .run(&generate_mix_rules());

    runner.print_report();

    let input_space = input_space.iter().cloned().collect::<HashSet<_>>();

    let extractor = Extractor::new(
        &runner.egraph,
        SillyCostFn::new(input_space, target_concentration, &runner.egraph),
    );

    let (cost, best_expr) = extractor.find_best(runner.roots[0]);
    let sequence = Sequence { cost, best_expr };
    Ok(sequence)
}

pub struct Sequence {
    pub cost: f64,
    pub best_expr: RecExpr<MixLang>,
}
