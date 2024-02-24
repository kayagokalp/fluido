mod cmd;

use std::str::FromStr;

use clap::Parser;
use cmd::Args;
use mixer_generator::concentration::Concentration;
use mixer_graph::{graph::Graph, parse::Expr};

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    let time_limit = args.time_limit;
    let target_concentration = args.target_concentration;
    let input_space = args
        .input_space
        .iter()
        .map(|input_concentration| Concentration::from_f64(*input_concentration))
        .collect();
    println!(
        "Starting to equality saturation, this will take ~{} seconds to finish.",
        time_limit
    );

    let generated_mixer_sequence =
        mixer_generator::saturate(target_concentration, time_limit, input_space)?;

    let cost = generated_mixer_sequence.cost;
    let expr = generated_mixer_sequence.best_expr;
    let parsed_expr = Expr::from_str(&format!("{expr}"))?;
    let graph = Graph::from(&parsed_expr);

    println!("best expr: {expr}");
    println!("cost: {cost}");
    let dot = graph.dot();
    println!("{dot}");
    let mut ir_builder = mixer_ir::ir_builder::IRBuilder::default();
    let ir_ops = ir_builder.build_ir(graph);
    for (op_index, op) in ir_ops.iter().enumerate() {
        println!("{} : {}", op_index, op)
    }
    Ok(())
}
