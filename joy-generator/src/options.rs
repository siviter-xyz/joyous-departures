/// Core options for generating goodbye messages
/// This is the Rust core interface - language bindings handle validation and fallbacks
#[derive(Debug, Clone)]
pub struct CoreGoodbyeOptions {
    /// ISO 639-1 language code with optional region (e.g., "en-GB", "en-US")
    /// Default: "en-GB"
    pub language_code: String,

    /// Template variable arguments
    /// Keys: "name", "location", "date", "time"
    /// Values: String replacements (empty means use default)
    pub template_args: std::collections::HashMap<String, String>,

    /// Whether to include emojis in the message
    /// Default: true
    pub use_emojis: bool,

    /// IANA timezone identifier (e.g., "Europe/London", "America/New_York")
    /// Default: "Europe/London"
    pub timezone: String,
}

impl Default for CoreGoodbyeOptions {
    fn default() -> Self {
        Self {
            language_code: "en-GB".to_string(),
            template_args: std::collections::HashMap::new(),
            use_emojis: true,
            timezone: "Europe/London".to_string(),
        }
    }
}

