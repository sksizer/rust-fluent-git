mod auth;
mod prompt;
mod session;

pub use auth::AuthError;
pub use prompt::PromptError;
pub use session::SessionError;

pub use fluent_core::CommandError;

/// Umbrella error for `?` propagation across multiple claude operations.
#[derive(Debug, thiserror::Error)]
pub enum ClaudeError {
    #[error(transparent)]
    Prompt(#[from] PromptError),
    #[error(transparent)]
    Session(#[from] SessionError),
    #[error(transparent)]
    Auth(#[from] AuthError),
}
