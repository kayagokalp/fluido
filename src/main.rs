mod cmd;
mod concentration;

use clap::Parser;
use cmd::Args;
use concentration::Concentration;
use egg::{rewrite as rw, *};
use std::{collections::HashSet, hash::Hash, time::Duration};

define_language! {
    enum MixLang {
        "mix" = Mix([Id; 2]),
        Num(Concentration),
        "+" = Add([Id; 2]),
        "/" = Div([Id; 2]),
        "-" = Sub([Id; 2]),
    }
}
#[derive(Default)]
struct ArithmeticAnalysis;

impl Analysis<MixLang> for ArithmeticAnalysis {
    type Data = Option<i64>; // Possible to store computed concentration or target info

    fn make(egraph: &EGraph<MixLang, Self>, enode: &MixLang) -> Self::Data {
        match enode {
            MixLang::Mix(_) => None,
            MixLang::Num(num) => Some(num.wrapped),
            MixLang::Add(add) => {
                let node_a = add[0];
                let node_b = add[1];

                let node_a_num = egraph[node_a].data.as_ref();
                let node_b_num = egraph[node_b].data.as_ref();

                node_a_num.and_then(|node_a| node_b_num.map(|node_b| node_a + node_b))
            }
            MixLang::Div(div) => {
                let node_a = div[0];
                let node_b = div[1];

                let node_a_num = egraph[node_a].data.as_ref();
                let node_b_num = egraph[node_b].data.as_ref();

                node_a_num.and_then(|node_a| node_b_num.map(|node_b| node_a / node_b))
            }
            MixLang::Sub(sub) => {
                let node_a = sub[0];
                let node_b = sub[1];

                let node_a_num = egraph[node_a].data.as_ref();
                let node_b_num = egraph[node_b].data.as_ref();

                node_a_num.and_then(|node_a| node_b_num.map(|node_b| node_a - node_b))
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
        if let Some(data) = egraph[id].data {
            let added = egraph.add(MixLang::Num(Concentration { wrapped: data }));
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

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;
    handle_args(args);
    Ok(())
}

fn handle_args(args: Args) {
    let start = format!("({})", args.target_concentration).parse().unwrap();

    let time_limit = args.time_limit;

    println!(
        "Starting to equality saturation, this will take ~{} seconds",
        time_limit
    );
    let runner: Runner<MixLang, ArithmeticAnalysis, ()> = Runner::new(ArithmeticAnalysis)
        .with_expr(&start)
        .with_node_limit(10000000000000000)
        .with_iter_limit(100000)
        .with_time_limit(Duration::from_secs(time_limit))
        .run(&generate_mix_rules());

    runner.print_report();

    let input_space = args
        .input_space
        .iter()
        .map(|concentration| Concentration::from_f64(*concentration))
        .collect();

    let extractor = Extractor::new(
        &runner.egraph,
        SillyCostFn::new(
            input_space,
            Concentration::from_f64(args.target_concentration),
        ),
    );
    let (cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("Optimized sequence: {}", best_expr);
    println!("Cost: {:?}", cost);
}
