use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("agent config error")]
    Config,
    #[error("open ai error : `{0}`")]
    OpenAI(String),
    #[error("open ai returned no text response")]
    NoTextResponse,
}
