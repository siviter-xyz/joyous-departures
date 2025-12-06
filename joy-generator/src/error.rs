use thiserror::Error;

/// Errors that can occur during message generation
#[derive(Debug, Error)]
pub enum GoodbyeError {
    /// Failed to load or decompress the message corpus
    #[error("Failed to load corpus: {0}")]
    CorpusLoadError(String),

    /// Invalid language code provided
    #[error("Invalid language code: {0}")]
    InvalidLanguageCodeError(String),

    /// Invalid timezone identifier provided
    #[error("Invalid timezone: {0}")]
    InvalidTimezoneError(String),

    /// Template variable replacement failed
    #[error("Template variable error: {0}")]
    TemplateVariableError(String),
}
