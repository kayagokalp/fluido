mod error;
use fluido_parse::parser::Parse;
use fluido_types::{
    concentration::Concentration,
    error::{
        FluidoError, IRGenerationError, InterefenceGraphGenerationError, MixerGenerationError,
    },
    expr::Expr,
};
use mixer_generator::Sequence;
use mixer_ir::{
    analysis::liveness::LivenessAnalysis,
    graph::Graph,
    ir::IROp,
    ir_builder::IRBuilder,
    pass_manager::IRPassManager,
    regalloc::interference_graph::{InterferenceGraph, InterferenceGraphBuilder},
};

/// A mixer generator for a specific target concentration from a given input space.
pub struct MixerDesign {
    mixer_expr: String,
    cost: f64,
    storage_units_needed: u64,
}

impl MixerDesign {
    pub fn mixer_expr(&self) -> &str {
        &self.mixer_expr
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }

    pub fn storage_units_needed(&self) -> u64 {
        self.storage_units_needed
    }
}

/// General configuration for fluido. Contains configuration settings for:
///  - Mixer generation
///  - Logging
#[derive(Debug, Clone)]
pub struct Config {
    generation: MixerGenerationConfig,
    logging: LogConfig,
}

impl Config {
    pub fn new(generation: MixerGenerationConfig, logging: LogConfig) -> Self {
        Self {
            generation,
            logging,
        }
    }
}

/// Settings for controlling various logging options.
#[derive(Debug, Clone)]
pub struct LogConfig {
    show_mixer_graph: bool,
    show_ir: bool,
    show_liveness: bool,
    show_interference_graph: bool,
}

impl LogConfig {
    pub fn new(
        show_mixer_graph: bool,
        show_ir: bool,
        show_liveness: bool,
        show_interference_graph: bool,
    ) -> Self {
        Self {
            show_mixer_graph,
            show_ir,
            show_liveness,
            show_interference_graph,
        }
    }
}

/// Different types of mixer generation handlers.
///
/// Currently fluido only supports equality saturation for mixer generation but it will eventually
/// add support for heuristics to generate initial mixer.
#[derive(Debug, Clone)]
pub enum MixerGenerator {
    EquailtySaturation,
}

#[derive(Debug, Clone)]
pub struct MixerGenerationConfig {
    time_limit: u64,
    generator: MixerGenerator,
}

impl MixerGenerationConfig {
    pub fn new(time_limit: u64, generator: MixerGenerator) -> Self {
        Self {
            time_limit,
            generator,
        }
    }
}

/// Generate a mixer for the target_concentration from input space.
fn generate_mixer_sequence(
    target_concentration: Concentration,
    input_space: &[Concentration],
    time_limit: u64,
    mixer_generator: MixerGenerator,
) -> Result<Sequence, MixerGenerationError> {
    match mixer_generator {
        MixerGenerator::EquailtySaturation => {
            let generated_mixer_sequence =
                mixer_generator::saturate(target_concentration, time_limit, input_space)?;
            Ok(generated_mixer_sequence)
        }
    }
}

/// Generates a `mixer-graph` from expr.
fn generate_graph(sequence: Sequence) -> Result<Graph, IRGenerationError> {
    let best_expr = sequence.best_expr;
    let expr_str = format!("{best_expr}");
    let expr = Expr::parse(&expr_str)?;
    Ok(Graph::from(&expr))
}

/// Generates interference graph from flat ir.
fn generate_interference_graph(
    ir_ops: Vec<IROp>,
    show_liveness: bool,
) -> Result<InterferenceGraph, InterefenceGraphGenerationError> {
    let mut ir_pass_manager = IRPassManager::new(ir_ops.clone(), vec![]);
    let liveness_analysis = LivenessAnalysis::default();
    ir_pass_manager.register_analysis_pass(&liveness_analysis);

    let analysis_results = ir_pass_manager.apply_analysis_passes();
    let liveness_result = &analysis_results
        .get("liveness")
        .ok_or(InterefenceGraphGenerationError::MissingLivenessAnalysis)?;
    if show_liveness {
        // Print liveness analysis result with flat-ir next to it.
        println!("ix  |  ir  |  live vreg set |");
        for (ix, (ir, liveset)) in ir_ops.iter().zip(&liveness_result.sets_per_ir).enumerate() {
            println!("{} : {} --- {:?}", ix, ir, liveset)
        }
    }

    let intereference_graph_builder = InterferenceGraphBuilder::new(&liveness_result.sets_per_ir);
    let interference_graph = intereference_graph_builder.build();

    Ok(interference_graph)
}

/// Searches a mixer design which is:
///  1- Valid in terms of the inputs it is using.
///  2- Uses minimum number of storage units. (IN-PROGRESS)
pub fn search_mixer_design(
    config: Config,
    target_concentration: Concentration,
    input_space: &[Concentration],
) -> Result<MixerDesign, FluidoError> {
    let mixer_generator = config.generation.generator;
    let time_limit = config.generation.time_limit;

    let mixer_sequence = generate_mixer_sequence(
        target_concentration,
        input_space,
        time_limit,
        mixer_generator,
    )?;

    let expr_str = format!("{}", mixer_sequence.best_expr);
    let cost = mixer_sequence.cost;

    let graph = generate_graph(mixer_sequence)?;
    if config.logging.show_mixer_graph {
        println!("{}", graph.dot());
    }

    let mut ir_builder = IRBuilder::default();
    let ir_ops = ir_builder.build_ir(graph);
    if config.logging.show_ir {
        for (op_index, op) in ir_ops.iter().enumerate() {
            println!("{} : {}", op_index, op)
        }
    }

    let interference_graph = generate_interference_graph(ir_ops, config.logging.show_liveness)?;
    if config.logging.show_interference_graph {
        println!("{}", interference_graph.dot());
    }

    let min_needed_color = interference_graph.find_min_color_count();

    let mixer_design = MixerDesign {
        mixer_expr: expr_str,
        cost,
        storage_units_needed: min_needed_color,
    };
    Ok(mixer_design)
}
