use egg::{rewrite as rw, *};
use fluido_types::{concentration::Concentration, error::MixerGenerationError};
use std::{collections::HashSet, hash::Hash, time::Duration};

define_language! {
    pub enum MixLang {
        "mix" = Mix([Id; 2]),
        Num(Concentration),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
    }
}
#[derive(Default)]
struct ArithmeticAnalysis;

impl Analysis<MixLang> for ArithmeticAnalysis {
    type Data = Option<Concentration>;

    fn make(egraph: &EGraph<MixLang, Self>, enode: &MixLang) -> Self::Data {
        match enode {
            MixLang::Mix(_) => None,
            MixLang::Num(num) => Some(num.clone()),
            MixLang::Add(add) => {
                let node_a = add[0];
                let node_b = add[1];

                let node_a_num = egraph[node_a].data.as_ref();
                let node_b_num = egraph[node_b].data.as_ref();

                node_a_num
                    .and_then(|node_a| node_b_num.map(|node_b| node_a.clone() + node_b.clone()))
            }
            MixLang::Sub(sub) => {
                let node_a = sub[0];
                let node_b = sub[1];

                let node_a_num = egraph[node_a].data.as_ref();
                let node_b_num = egraph[node_b].data.as_ref();

                node_a_num
                    .and_then(|node_a| node_b_num.map(|node_b| node_a.clone() - node_b.clone()))
            }
        }
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(*a, b, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph<MixLang, Self>, id: Id) {
        if let Some(data) = &egraph[id].data {
            let added = egraph.add(MixLang::Num(data.clone()));
            egraph.union(id, added);
        }
    }
}

fn generate_mix_rules() -> Vec<Rewrite<MixLang, ArithmeticAnalysis>> {
    vec![
        rw!("differentiate-mixer";
            "(mix ?a ?b)" => "(mix (- ?a 0.001) (+ ?b 0.001))"),
        rw!("differentiate-mixer2";
            "(mix ?a ?b)" => "(mix (+ ?a 0.001) (- ?b 0.001))"),
        rw!("expand-num";
            "(?a)" => "(mix ?a ?a)"),
    ]
}

struct SillyCostFn {
    input_space: HashSet<Concentration>,
    target: Concentration,
}

impl SillyCostFn {
    fn new(input_space: HashSet<Concentration>, target: Concentration) -> Self {
        Self {
            input_space,
            target,
        }
    }
}
impl CostFunction<MixLang> for SillyCostFn {
    type Cost = f64;

    fn cost<C>(&mut self, enode: &MixLang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let op_cost = match enode {
            MixLang::Mix(_) => 0.0,
            MixLang::Num(num) => {
                if *num == self.target {
                    f64::MAX
                } else if self.input_space.contains(num) {
                    0.0
                } else {
                    let closest_val = self
                        .input_space
                        .iter()
                        .map(|input| (input.clone() - num.clone()).wrapped.abs())
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
    let start = format!("({})", target_concentration)
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
        SillyCostFn::new(input_space, target_concentration),
    );

    let (cost, best_expr) = extractor.find_best(runner.roots[0]);
    let sequence = Sequence { cost, best_expr };
    Ok(sequence)
}

pub struct Sequence {
    pub cost: f64,
    pub best_expr: RecExpr<MixLang>,
}
