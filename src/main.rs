use egg::{rewrite as rw, *};
use ordered_float::NotNan;

type Concentration = ordered_float::NotNan<f64>;

// Define your language, including arithmetic operations
define_language! {
    enum MixLang {
        "mix" = Mix([Id; 2]),
        Num(Concentration),
        "+" = Add([Id; 2]),
        "/" = Div([Id; 2]),
    }
}

#[derive(Default)]
struct ArithmeticAnalysis;

impl Analysis<MixLang> for ArithmeticAnalysis {
    type Data = Option<NotNan<f64>>; // Possible to store computed concentration or target info

    fn make(egraph: &EGraph<MixLang, Self>, enode: &MixLang) -> Self::Data {
        match enode {
            MixLang::Mix(_) => None,
            MixLang::Num(num) => Some(*num),
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
        }
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(*a, *b, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph<MixLang, Self>, id: Id) {
        if let Some(data) = egraph[id].data {
            let added = egraph.add(MixLang::Num(data));
            egraph.union(id, added);
        }
    }
}

fn generate_mix_rules() -> Vec<Rewrite<MixLang, ArithmeticAnalysis>> {
    vec![rw!("combine_two"; 
            "(mix ?a ?b)" => "(/ (+ ?a ?b) 2.0)")]
}

fn main() {
    let start = "(mix 0.5 1.0)".parse().unwrap();

    let runner: Runner<MixLang, ArithmeticAnalysis, ()> = Runner::new(ArithmeticAnalysis)
        .with_expr(&start)
        .with_iter_limit(10)
        .run(&generate_mix_rules());

    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("Optimized sequence: {:?}", best_expr);
    println!("Cost: {:?}", cost);
}
