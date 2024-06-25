use egg::{rewrite as rw, *};
use fluido_types::{
    error::MixerGenerationError,
    fluid::{Concentration, Fluid},
    number::SaturationNumber,
};
use std::{collections::HashSet, time::Duration};

define_language! {
    // TODO: `define_language!` macro does not support generics, fix this.
    pub enum MixLang<T: SaturationNumber> {
        Number(T),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "\\" = Div([Id; 2]),
        "*" = Mult([Id; 2]),
        "mix" = Mix([Id; 2]),
        "fluid" = Fluid([Id; 2]),
    }
}
#[derive(Default)]
struct ArithmeticAnalysis;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ArithmeticAnalysisPayload<T: SaturationNumber> {
    Number(T),
    Fluid(Fluid<T>),
    None,
}

impl<T: SaturationNumber> ArithmeticAnalysisPayload<T> {
    pub fn expect_number(self) -> Option<T> {
        match self {
            ArithmeticAnalysisPayload::Number(nm) => Some(nm),
            _ => None,
        }
    }
}

impl<T: SaturationNumber> MixLang<T> {
    pub fn expect_number(self) -> Option<T> {
        match self {
            MixLang::Number(nm) => Some(nm),
            _ => None,
        }
    }
}

impl<T: SaturationNumber> Analysis<MixLang<T>> for ArithmeticAnalysis {
    type Data = ArithmeticAnalysisPayload<T>;

    fn make(egraph: &mut EGraph<MixLang<T>, Self>, enode: &MixLang<T>) -> Self::Data {
        match enode {
            MixLang::Mix(mix) => {
                let fluid_a = &egraph[mix[0]];
                let fluid_b = &egraph[mix[1]];

                let fluid_a_conc_id = &fluid_a.nodes[0].children()[0];
                let fluid_a_vol_id = &fluid_a.nodes[0].children()[1];
                let fluid_a_conc = &egraph[*fluid_a_conc_id].nodes[0].clone().expect_number();
                let fluid_a_vol = &egraph[*fluid_a_vol_id].nodes[0].clone().expect_number();

                let fluid_b_conc_id = &fluid_b.nodes[0].children()[0];
                let fluid_b_vol_id = &fluid_b.nodes[0].children()[1];
                let fluid_b_conc = &egraph[*fluid_b_conc_id].nodes[0].clone().expect_number();
                let fluid_b_vol = &egraph[*fluid_b_vol_id].nodes[0].clone().expect_number();

                if let (
                    Some(fluid_a_conc),
                    Some(fluid_a_vol),
                    Some(fluid_b_conc),
                    Some(fluid_b_vol),
                ) = (fluid_a_conc, fluid_a_vol, fluid_b_conc, fluid_b_vol)
                {
                    let fluid_a = Fluid::new(*fluid_a_conc, *fluid_a_vol);
                    let fluid_b = Fluid::new(*fluid_b_conc, *fluid_b_vol);

                    let mixed_fluid = fluid_a.mix(&fluid_b);
                    ArithmeticAnalysisPayload::Fluid(mixed_fluid)
                } else {
                    ArithmeticAnalysisPayload::None
                }
            }
            MixLang::Fluid(fl) => {
                let node_a_id = fl[0];
                let node_b_id = fl[1];

                let node_a = &egraph[node_a_id].data.clone().expect_number();
                let node_b = &egraph[node_b_id].data.clone().expect_number();

                if let (Some(conc), Some(vol)) = (node_a, node_b) {
                    let fl = Fluid::new(*conc, *vol);
                    ArithmeticAnalysisPayload::Fluid(fl)
                } else {
                    ArithmeticAnalysisPayload::None
                }
            }
            MixLang::Number(nm) => ArithmeticAnalysisPayload::Number(*nm),
            MixLang::Add(add) => {
                let node_a_id = add[0];
                let node_b_id = add[1];

                let node_a = &egraph[node_a_id].data;
                let node_b = &egraph[node_b_id].data;

                let val_a = node_a.clone().expect_number().unwrap();
                let val_b = node_b.clone().expect_number().unwrap();
                let result = val_a + val_b;
                ArithmeticAnalysisPayload::Number(result)
            }
            MixLang::Sub(sub) => {
                let node_a_id = sub[0];
                let node_b_id = sub[1];

                let node_a = &egraph[node_a_id].data;
                let node_b = &egraph[node_b_id].data;

                let val_a = node_a.clone().expect_number().unwrap();
                let val_b = node_b.clone().expect_number().unwrap();

                let result = val_a - val_b;
                ArithmeticAnalysisPayload::Number(result)
            }
            MixLang::Div(div) => {
                let node_a_id = div[0];
                let node_b_id = div[1];

                let node_a = egraph[node_a_id].clone().data;
                let node_b = egraph[node_b_id].clone().data;

                let val_a = node_a.clone().expect_number().unwrap();
                let val_b = node_b.clone().expect_number().unwrap();
                let result = val_a / val_b;
                ArithmeticAnalysisPayload::Number(result)
            }
            MixLang::Mult(mult) => {
                let node_a_id = mult[0];
                let node_b_id = mult[1];

                let node_a = egraph[node_a_id].clone().data;
                let node_b = egraph[node_b_id].clone().data;

                let val_a = node_a.clone().expect_number().unwrap();
                let val_b = node_b.clone().expect_number().unwrap();
                let result = val_a * val_b;
                ArithmeticAnalysisPayload::Number(result)
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

    fn modify(egraph: &mut EGraph<MixLang<T>, Self>, id: Id) {
        if let ArithmeticAnalysisPayload::Fluid(fl) = egraph[id].data.clone() {
            let concentration = fl.concentration();
            let concentration_node = egraph.add(MixLang::Number(*concentration));
            let volume = fl.unit_volume();
            let volume_node = egraph.add(MixLang::Number(*volume));
            let added = egraph.add(MixLang::Fluid([concentration_node, volume_node]));
            egraph.union(id, added);
        }
    }
}

pub struct OpCost<'a, T: SaturationNumber> {
    target: T,
    input_space: HashSet<T>,
    egraph: &'a EGraph<MixLang<T>, ArithmeticAnalysis>,
}

impl<'a, T: SaturationNumber> OpCost<'a, T> {
    pub(crate) fn new(
        target: T,
        input_space: HashSet<T>,
        egraph: &'a EGraph<MixLang<T>, ArithmeticAnalysis>,
    ) -> Self {
        Self {
            target,
            input_space,
            egraph,
        }
    }

    fn is_fluid_in_input_space(&self, fluid: &Fluid<T>) -> bool {
        self.input_space.contains(fluid.concentration())
    }

    fn is_direct_fluid_available(&self, fluid: &Fluid<T>) -> bool {
        self.is_fluid_in_input_space(fluid)
    }

    fn proximity_cost(&self, conc: &T) -> f64 {
        let mut min = 1.0;
        for val in self.input_space.iter() {
            let diff = *conc - *val;
            let diff: f64 = diff.into();
            let diff = diff.abs();
            if diff < min {
                min = diff;
            }
        }
        min
    }
}

impl<'a, T: SaturationNumber> egg::CostFunction<MixLang<T>> for OpCost<'a, T> {
    type Cost = f64;

    fn cost<C>(&mut self, enode: &MixLang<T>, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let base_cost = match enode {
            MixLang::Number(_) => 0.0,
            MixLang::Add(_) => 100.0,
            MixLang::Sub(_) => 100.0,
            MixLang::Div(_) => 100.0,
            MixLang::Mult(_) => 100.0,
            MixLang::Mix(_) => 1.0,
            MixLang::Fluid(fl) => {
                let conc_id = fl[0];
                let vol_id = fl[1];

                if let (Some(conc), Some(vol)) = (
                    self.egraph[conc_id].data.clone().expect_number(),
                    self.egraph[vol_id].data.clone().expect_number(),
                ) {
                    let fluid = Fluid::new(conc, vol);
                    let concentration = fluid.concentration();
                    if self.is_direct_fluid_available(&fluid) {
                        0.0
                    } else if self.target == *concentration {
                        f64::MAX
                    } else {
                        // TODO: move scaling 1/concentration_epsilon multiplication to number impl.
                        self.proximity_cost(concentration) // * (1.0 / Concentration::EPSILON)
                    }
                } else {
                    1000.0
                }
            }
        };
        enode.fold(base_cost, |sum, id| sum + costs(id))
    }
}

fn generate_rewrite_rules<T: SaturationNumber + 'static>(
) -> Vec<Rewrite<MixLang<T>, ArithmeticAnalysis>> {
    vec![
        rw!("expand-fluid-to-mix";
            "(fluid ?a ?b)" => "(mix (fluid ?a (\\ ?b 2.0)) (fluid ?a (\\ ?b 2.0)))"
            if (volume_valid("?b"))),
        rw!("diff-mixers-l-0.01";
            "(mix (fluid ?a ?b) (fluid ?c ?b))" => "(mix (fluid (+ ?a 0.01) ?b) (fluid (- ?c 0.01) ?b))"
        if concentration_valid("?a", Op::Add, "?c", Op::Remove, 0.01)),
        rw!("diff-mixers-l-0.1";
            "(mix (fluid ?a ?b) (fluid ?c ?b))" => "(mix (fluid (+ ?a 0.1) ?b) (fluid (- ?c 0.1) ?b))"
            if concentration_valid("?a", Op::Add, "?c", Op::Remove, 0.1)),
        rw!("mixer-assoc";
            "(mix (fluid ?a ?b) (fluid ?c ?d))" => "(mix (fluid ?c ?d) (fluid ?a ?b))"),
        rw!("mixer-compress-with-0";
            "(mix (mix (fluid ?a ?b) (fluid 0.0 ?b)) (fluid 0.0 ?c))" => "(mix (fluid ?a (\\ ?b 2.0)) (fluid 0.0 (* 3.0 (\\ ?b 2.0))))"
        if volume_multiple("?b", "?c", 0.5)),
    ]
}

fn volume_multiple<T: SaturationNumber>(
    vol_a: &'static str,
    vol_b: &'static str,
    multiple: f64,
) -> impl Fn(&mut EGraph<MixLang<T>, ArithmeticAnalysis>, Id, &Subst) -> bool {
    let var_vol_a: Var = vol_a.parse().unwrap();
    let var_vol_b: Var = vol_b.parse().unwrap();
    move |egraph, _, subst| {
        let vol_a = subst[var_vol_a];
        let vol_node_a = &egraph[vol_a];
        let vol_a = vol_node_a.data.clone().expect_number().unwrap();
        let vol_a_float: f64 = vol_a.into();

        let vol_b = subst[var_vol_b];
        let vol_node_b = &egraph[vol_b];
        let vol_b = vol_node_b.data.clone().expect_number().unwrap();
        let vol_b_float: f64 = vol_b.into();

        let div = vol_a_float / vol_b_float;
        div == multiple
    }
}

fn volume_valid<T: SaturationNumber>(
    vol: &'static str,
) -> impl Fn(&mut EGraph<MixLang<T>, ArithmeticAnalysis>, Id, &Subst) -> bool {
    let var_vol: Var = vol.parse().unwrap();
    move |egraph, _, subst| {
        let vol = subst[var_vol];
        let vol_node = &egraph[vol];
        let vol = vol_node.data.clone().expect_number().unwrap();
        let vol_float: f64 = vol.into();
        let two = T::from(2.0);
        let res = vol / two;
        let res: f64 = res.into();

        let res_float = vol_float / 2.0;

        // if division starts to loose precision, we want to stop dividing
        let precision_preserved = res == res_float;

        // Physically we know that a volume is positive.
        let volume_is_positive = res > 0.0;

        volume_is_positive && precision_preserved
    }
}

enum Op {
    Add,
    Remove,
}

fn concentration_valid<T: SaturationNumber>(
    concentration_a: &'static str,
    op_a: Op,
    concentration_b: &'static str,
    op_b: Op,
    step: f64,
) -> impl Fn(&mut EGraph<MixLang<T>, ArithmeticAnalysis>, Id, &Subst) -> bool {
    let var_concentration_a: Var = concentration_a.parse().unwrap();
    let var_concentration_b: Var = concentration_b.parse().unwrap();
    move |egraph, _, subst| {
        let conc_a = subst[var_concentration_a];
        let conc_node_a = &egraph[conc_a];
        let concentration_a = conc_node_a.data.clone().expect_number().unwrap();
        let concentration_a: f64 = concentration_a.into();

        let res_a = match op_a {
            Op::Add => concentration_a + step,
            Op::Remove => concentration_a - step,
        };
        let concentration_a = Concentration::from(res_a);

        let conc_b = subst[var_concentration_b];
        let conc_node_b = &egraph[conc_b];
        let concentration_b = conc_node_b.data.clone().expect_number().unwrap();
        let concentration_b: f64 = concentration_b.into();
        let res_b = match op_b {
            Op::Add => concentration_b + step,
            Op::Remove => concentration_b - step,
        };
        let concentration_b = Concentration::from(res_b);

        concentration_a.valid() && concentration_b.valid()
    }
}

fn normalize_expr_by_min_volume<T: SaturationNumber>(expr: &RecExpr<MixLang<T>>) -> String {
    // Find the smallest volume in the expression
    let mut min_volume: Option<f64> = None;
    for node in expr.as_ref() {
        if let MixLang::Fluid(fluid) = node {
            if let MixLang::Number(vol) = &expr[fluid[1]] {
                let vol_float: f64 = (*vol).into();
                min_volume = Some(min_volume.map_or(vol_float, |min| min.min(vol_float)));
            }
        }
    }

    // If there's no fluid node or min_volume is still None, return the original expression as a string
    let min_volume = match min_volume {
        Some(vol) => vol,
        None => return format!("{}", expr),
    };

    // Helper function to format the nodes
    fn format_node<T: SaturationNumber>(
        expr: &RecExpr<MixLang<T>>,
        id: Id,
        min_volume: f64,
    ) -> String {
        match &expr[id] {
            MixLang::Fluid(fluid) => {
                let conc = &expr[fluid[0]];
                let vol = &expr[fluid[1]];
                if let MixLang::Number(vol) = vol {
                    let vol_float: f64 = (*vol).into();
                    let normalized_vol = vol_float / min_volume;
                    if let MixLang::Number(conc) = conc {
                        return format!("(fluid {} {})", conc, normalized_vol);
                    }
                }
                String::new()
            }
            MixLang::Mix(mix) => {
                let left = format_node(expr, mix[0], min_volume);
                let right = format_node(expr, mix[1], min_volume);
                format!("(mix {} {})", left, right)
            }
            MixLang::Add(add) => {
                let left = format_node(expr, add[0], min_volume);
                let right = format_node(expr, add[1], min_volume);
                format!("(+ {} {})", left, right)
            }
            MixLang::Sub(sub) => {
                let left = format_node(expr, sub[0], min_volume);
                let right = format_node(expr, sub[1], min_volume);
                format!("(- {} {})", left, right)
            }
            MixLang::Div(div) => {
                let left = format_node(expr, div[0], min_volume);
                let right = format_node(expr, div[1], min_volume);
                format!("(/ {} {})", left, right)
            }
            MixLang::Mult(mult) => {
                let left = format_node(expr, mult[0], min_volume);
                let right = format_node(expr, mult[1], min_volume);
                format!("(* {} {})", left, right)
            }
            MixLang::Number(lf) => format!("{}", lf),
        }
    }

    // Format the root node
    let root_id = expr.as_ref().len() - 1;
    format_node(expr, Id::from(root_id), min_volume)
}

/// Saturate to find out an optimized sequence according to the cost function.
pub fn saturate<T: SaturationNumber + 'static>(
    target_concentration: T,
    time_limit: u64,
    input_space: &[Fluid<T>],
) -> Result<Sequence<T>, MixerGenerationError> {
    let mut initial_egraph = EGraph::new(ArithmeticAnalysis);
    let target_node = format!("(fluid {} {}/1)", target_concentration, f64::MAX)
        .parse::<RecExpr<MixLang<T>>>()
        .map_err(|e| {
            dbg!(e);
            MixerGenerationError::FailedToParseTarget(format!("{}", target_concentration))
        })?;

    let target = initial_egraph.add_expr(&target_node);

    let input_space = input_space
        .iter()
        .map(|fluid| fluid.concentration())
        .cloned()
        .collect::<HashSet<_>>();

    let runner: Runner<MixLang<T>, ArithmeticAnalysis, ()> = Runner::new(ArithmeticAnalysis)
        .with_egraph(initial_egraph)
        .with_node_limit(10000000000000000)
        .with_iter_limit(100000)
        .with_time_limit(Duration::from_secs(time_limit))
        .run(&generate_rewrite_rules());

    runner.print_report();

    let extractor = Extractor::new(
        &runner.egraph,
        OpCost::new(target_concentration, input_space, &runner.egraph),
    );

    let (cost, best_expr) = extractor.find_best(target);
    let best_expr_normalized_str = normalize_expr_by_min_volume(&best_expr);
    let best_expr_normalized = best_expr_normalized_str
        .parse::<RecExpr<MixLang<T>>>()
        .map_err(|e| MixerGenerationError::SaturationError(e.to_string()))?;

    println!("{best_expr_normalized} cost {cost}");
    let sequence = Sequence {
        cost,
        best_expr: best_expr_normalized,
    };
    Ok(sequence)
}

pub struct Sequence<T: SaturationNumber> {
    pub cost: f64,
    pub best_expr: RecExpr<MixLang<T>>,
}
