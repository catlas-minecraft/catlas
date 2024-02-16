use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ManagerError>;

#[derive(Debug, Error)]
pub enum ManagerError {
    #[error("Failed open region file:\n{0}\"")]
    FailedOpenRegion(#[source] io::Error),

    #[error("Failed load region file:\n{0}\"")]
    FailedLoadRegion(#[source] fastanvil::Error)
}