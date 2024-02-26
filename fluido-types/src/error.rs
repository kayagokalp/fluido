use thiserror::Error;

#[derive(Error, Debug)]
pub enum MixerGenerationError {
    #[error("Saturation error while generating the mixer space: {0}")]
    SaturationError(String),
}

#[derive(Error, Debug)]
pub enum IRGenerationError {
    #[error("{0}")]
    ParseError(String),
}
