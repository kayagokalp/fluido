use std::time::Duration;

use egg::{rewrite as rw, *};
use rand::Rng;

type Concentration = ordered_float::NotNan<f64>;

// Define your language, including arithmetic operations
define_language! {
    enum MixLang {
        "mix" = Mix([Id; 2]),
        "randnumpair" = RandNum([Id; 1]),
        Num(Concentration),
        "+" = Add([Id; 2]),
        "/" = Div([Id; 2]),
    }
}

#[derive(Debug, Clone)]
enum ArithmeticAnalysisData {
    Constant(Concentration),
    Tuple((Concentration, Concentration)),
}

impl PartialEq for ArithmeticAnalysisData {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.001;
        match (self, other) {
            (Self::Constant(l0), Self::Constant(r0)) => (l0 - r0).abs() < epsilon,
            (Self::Tuple(l0), Self::Tuple(r0)) => {
                (l0.0 - r0.0).abs() < epsilon && (l0.1 - r0.1).abs() < epsilon
            }
            _ => false,
        }
    }
}

impl Eq for ArithmeticAnalysisData {}

impl ArithmeticAnalysisData {
    fn as_constant(&self) -> Option<Concentration> {
        match self {
            ArithmeticAnalysisData::Constant(constant) => Some(*constant),
            ArithmeticAnalysisData::Tuple(_) => None,
        }
    }
}

#[derive(Default)]
struct ArithmeticAnalysis;

impl Analysis<MixLang> for ArithmeticAnalysis {
    type Data = Option<ArithmeticAnalysisData>;

    fn make(egraph: &EGraph<MixLang, Self>, enode: &MixLang) -> Self::Data {
        match enode {
            MixLang::Mix(_) => None,
            MixLang::Num(num) => Some(ArithmeticAnalysisData::Constant(*num)),
            MixLang::Add(add) => {
                let node_a = add[0];
                let node_b = add[1];

                let node_a_num = egraph[node_a]
                    .data
                    .as_ref()
                    .and_then(ArithmeticAnalysisData::as_constant);
                let node_b_num = egraph[node_b]
                    .data
                    .as_ref()
                    .and_then(ArithmeticAnalysisData::as_constant);

                node_a_num.and_then(|node_a| {
                    node_b_num.map(|node_b| ArithmeticAnalysisData::Constant(node_a + node_b))
                })
            }
            MixLang::Div(div) => {
                let node_a = div[0];
                let node_b = div[1];

                let node_a_num = egraph[node_a]
                    .data
                    .as_ref()
                    .and_then(ArithmeticAnalysisData::as_constant);
                let node_b_num = egraph[node_b]
                    .data
                    .as_ref()
                    .and_then(ArithmeticAnalysisData::as_constant);
                node_a_num.and_then(|node_a| {
                    node_b_num.map(|node_b| ArithmeticAnalysisData::Constant(node_a / node_b))
                })
            }
            MixLang::RandNum(target_val) => {
                let node_a = target_val[0];
                let node_a_num = egraph[node_a]
                    .data
                    .as_ref()
                    .and_then(ArithmeticAnalysisData::as_constant)
                    .unwrap();

                let mut rng = rand::thread_rng();
                let max = Concentration::min(node_a_num * 2.0, Concentration::new(1.0).unwrap());
                println!("{node_a_num:?}");
                let (random_num, compl_num) = if (max - 0.0).abs() >= 0.001 {
                    let random_num = rng.gen_range(0.0..*max);
                    let random_num = (random_num * 1000.0).round() / 1000.0;
                    let random_num = Concentration::new(random_num).unwrap();

                    let target_sum = node_a_num * 2.0;
                    let compl_num = target_sum - random_num;

                    (random_num, compl_num)
                } else {
                    let zero = Concentration::new(0.0).unwrap();
                    (zero, zero)
                };

                if *random_num > 1.0 || *compl_num > 1.0 {
                    return None;
                }

                Some(ArithmeticAnalysisData::Tuple((random_num, compl_num)))
            }
        }
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            //assert_eq!(*a, b, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph<MixLang, Self>, id: Id) {
        if let Some(data) = egraph[id].data.clone() {
            match data {
                ArithmeticAnalysisData::Constant(constant_data) => {
                    let added = egraph.add(MixLang::Num(constant_data));
                    egraph.union(id, added);
                }
                ArithmeticAnalysisData::Tuple(tuple) => {
                    let num1 = egraph.add(MixLang::Num(tuple.0));
                    let num2 = egraph.add(MixLang::Num(tuple.1));
                    let added = egraph.add(MixLang::Mix([num1, num2]));
                    println!("mix {} {}", tuple.0, tuple.1);
                    egraph[id].data = None;
                    egraph.union(id, added);
                }
            }
        }
    }
}

struct SillyCostFn {
    target: Concentration,
    input_space: Vec<Concentration>,
}

impl SillyCostFn {
    fn new(target: Concentration, input_space: Vec<Concentration>) -> Self {
        Self {
            target,
            input_space,
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
                } else {
                    let closest_val = self
                        .input_space
                        .iter()
                        .map(|input| Concentration::new((input - num).abs()).unwrap())
                        .min();
                    *closest_val.unwrap()
                }
            }
            _ => f64::MAX,
        };

        enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}

fn generate_mix_rules() -> Vec<Rewrite<MixLang, ArithmeticAnalysis>> {
    vec![
        rw!("expand_num"; 
            "(?a)" => "(randnumpair ?a)"),
        rw!("combine_two"; 
            "(mix ?a ?b)" => "(/ (+ ?a ?b) 2.0)"),
    ]
}

fn main() {
    let start = "(0.2)".parse().unwrap();

    let runner: Runner<MixLang, ArithmeticAnalysis, ()> = Runner::new(ArithmeticAnalysis)
        .with_explanations_enabled()
        .with_expr(&start)
        .with_time_limit(Duration::from_secs(20))
        .with_node_limit(1000000000000)
        .with_iter_limit(10000)
        .run(&generate_mix_rules());

    runner.print_report();
    let extractor = Extractor::new(
        &runner.egraph,
        SillyCostFn::new(
            Concentration::new(0.2).unwrap(),
            vec![
                Concentration::new(0.35).unwrap(),
                Concentration::new(0.05).unwrap(),
            ],
        ),
    );
    let (cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("Optimized sequence: {}", best_expr);
    println!("Cost: {:?}", cost);
}
