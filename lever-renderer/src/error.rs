use thiserror::Error;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("Shader compilation failed: {0}")]
    ShaderCompile(String),
    #[error("Program linking failed: {0}")]
    ProgramLink(String),
    #[error("OpenGL allocation failed: {0}")]
    GlAllocation(&'static str),
}
