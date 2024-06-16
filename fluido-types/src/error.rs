use crate::fluid::Concentration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MixerGenerationError {
    #[error("Saturation error while generating the mixer space: {0}")]
    SaturationError(String),
    #[error("Failed to parse target concentration (`{0}`) as a node.")]
    FailedToParseTarget(Concentration),
}

#[derive(Error, Debug)]
pub enum IRGenerationError {
    #[error("{0}")]
    ParseError(String),
}

#[derive(Error, Debug)]
pub enum InterefenceGraphGenerationError {
    #[error("Missing liveness analysis in the ir analysis results.")]
    MissingLivenessAnalysis,
}
#[derive(Error, Debug)]
pub enum FluidoError {
    #[error("{0}")]
    MixerGenerationError(MixerGenerationError),
    #[error("{0}")]
    IRGenerationError(IRGenerationError),
    #[error("{0}")]
    InterferenceGraphGenerationError(InterefenceGraphGenerationError),
}

impl From<MixerGenerationError> for FluidoError {
    fn from(value: MixerGenerationError) -> Self {
        Self::MixerGenerationError(value)
    }
}

impl From<IRGenerationError> for FluidoError {
    fn from(value: IRGenerationError) -> Self {
        Self::IRGenerationError(value)
    }
}

impl From<InterefenceGraphGenerationError> for FluidoError {
    fn from(value: InterefenceGraphGenerationError) -> Self {
        Self::InterferenceGraphGenerationError(value)
    }
}
