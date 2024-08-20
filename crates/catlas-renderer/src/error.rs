use thiserror::Error;

pub type Result<T> = std::result::Result<T, CatlasRenderError>;

#[derive(Error, Debug)]
pub enum CatlasRenderError {
    #[error("Failed read chunk: {}", .0)]
    ReadChunkError(#[source] fastanvil::Error),

    #[error("Failed deserialize chunk: {}", .0)]
    ChunkDerError(#[source] fastnbt::error::Error)
}
