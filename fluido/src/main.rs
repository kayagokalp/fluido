mod cmd;

use std::str::FromStr;

use clap::Parser;
use cmd::Args;
use mixer_generator::concentration::Concentration;
use mixer_graph::{graph::Graph, parse::Expr};
use mixer_ir::regalloc::interference_graph::InterferenceGraphBuilder;

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

    if args.show_dot {
        let dot = graph.dot();
        println!("{dot}");
    }

    let mut ir_builder = mixer_ir::ir_builder::IRBuilder::default();
    let ir_ops = ir_builder.build_ir(graph);

    if args.show_ir {
        for (op_index, op) in ir_ops.iter().enumerate() {
            println!("{} : {}", op_index, op)
        }
    }

    let mut ir_pass_manager = mixer_ir::pass_manager::IRPassManager::new(ir_ops.clone(), vec![]);
    // Register liveness analysis pass
    let liveness_analysis = mixer_ir::analysis::liveness::LivenessAnalysis::default();
    ir_pass_manager.register_analysis_pass(&liveness_analysis);
    let analysis_results = ir_pass_manager.apply_analysis_passes();
    let liveness_result = &analysis_results["liveness"];
    if args.show_liveness {
        // Print liveness analysis result with flat-ir next to it.
        println!("ix  |  ir  |  live vreg set |");
        for (ix, (ir, liveset)) in ir_ops.iter().zip(&liveness_result.sets_per_ir).enumerate() {
            println!("{} : {} --- {:?}", ix, ir, liveset)
        }
    }

    let interference_graph_builder = InterferenceGraphBuilder::new(&liveness_result.sets_per_ir);
    let interference_graph = interference_graph_builder.build();

    if args.show_interference {
        println!("{}", interference_graph.dot())
    }
    Ok(())
}
