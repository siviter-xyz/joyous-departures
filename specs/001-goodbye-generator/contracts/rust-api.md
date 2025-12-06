# Rust Core API Contract

**Crate**: `joy-generator`  
**Version**: 1.0.0  
**Date**: 2025-01-27

## Cargo.toml

```toml
[package]
name = "joy-generator"
version = "1.0.0"
edition = "2021"

[dependencies]
lz4 = "1.24"
chrono = "0.4"
chrono-tz = "0.9"
```

## Type Definitions

### CoreGoodbyeOptions

```rust
use std::collections::HashMap;

pub struct CoreGoodbyeOptions {
    /// ISO 639 language code (e.g., "en-GB", "en-US", "fr-FR")
    /// Defaults to "en-GB" if None
    pub language_code: Option<String>,
    
    /// Template variable replacements
    /// Key: variable name (e.g., "name", "location", "date", "time")
    /// Value: replacement text (e.g., "Alice", "Paris", "2025-01-27", "14:30")
    /// Defaults: name="Good Soul", location="The World", date=current date, time=current time
    pub template_args: HashMap<String, String>,
    
    /// Whether to include emojis in the message
    /// Defaults to true
    pub use_emojis: bool,
    
    /// IANA timezone identifier (e.g., "Europe/London", "America/New_York")
    /// Used for default values of {date} and {time} template variables
    /// Defaults to "Europe/London" if None
    pub timezone: Option<String>,
}

impl Default for CoreGoodbyeOptions {
    fn default() -> Self {
        Self {
            language_code: Some("en-GB".to_string()),
            template_args: HashMap::new(), // Defaults applied during template replacement
            use_emojis: true,
            timezone: Some("Europe/London".to_string()),
        }
    }
}
```

## Function Signature

### generate_goodbye

```rust
/// Generates a random warm sign-off message
/// 
/// # Arguments
/// 
/// * `options` - Configuration options for message generation
/// 
/// # Returns
/// 
/// A `Result` containing the generated message string, or an error
/// 
/// # Errors
/// 
/// Returns `CorpusLoadError` if the message corpus cannot be loaded
/// Returns `InvalidLanguageCodeError` if language_code format is invalid
/// 
/// # Examples
/// 
/// ```rust
/// use joy_generator::{generate_goodbye, CoreGoodbyeOptions};
/// use std::collections::HashMap;
/// 
/// // Basic usage
/// let options = CoreGoodbyeOptions::default();
/// let message = generate_goodbye(&options)?;
/// // "Wishing you a liberated day, Good Soul❤️"
/// 
/// // With custom name
/// let mut options = CoreGoodbyeOptions::default();
/// options.template_args.insert("name".to_string(), "Alice".to_string());
/// let message = generate_goodbye(&options)?;
/// // "Wishing you a liberated day, Alice❤️"
/// 
/// // With location and timezone
/// let mut options = CoreGoodbyeOptions::default();
/// options.template_args.insert("location".to_string(), "Paris".to_string());
/// options.timezone = Some("Europe/Paris".to_string());
/// let message = generate_goodbye(&options)?;
/// 
/// // Without emojis
/// let mut options = CoreGoodbyeOptions::default();
/// options.use_emojis = false;
/// let message = generate_goodbye(&options)?;
/// // "Wishing you a liberated day, Good Soul"
/// ```
pub fn generate_goodbye(options: &CoreGoodbyeOptions) -> Result<String, GoodbyeError>;
```

## Error Types

### GoodbyeError

```rust
#[derive(Debug, thiserror::Error)]
pub enum GoodbyeError {
    #[error("Failed to load message corpus: {0}")]
    CorpusLoadError(String),
    
    #[error("Invalid language code format: {0}")]
    InvalidLanguageCodeError(String),
    
    #[error("Template variable error: {0}")]
    TemplateVariableError(String),
}
```

## Module Structure

```rust
pub mod generate;
pub mod corpus;
pub mod template;
pub mod emoji;
pub mod datetime;
pub mod options;
pub mod error;

pub use generate::generate_goodbye;
pub use options::CoreGoodbyeOptions;
pub use error::GoodbyeError;
```

## Examples

### Basic Usage

```rust
use joy_generator::{generate_goodbye, CoreGoodbyeOptions};

let options = CoreGoodbyeOptions::default();
match generate_goodbye(&options) {
    Ok(message) => println!("{}", message),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Custom Options

```rust
use joy_generator::{generate_goodbye, CoreGoodbyeOptions};
use std::collections::HashMap;

let mut options = CoreGoodbyeOptions {
    language_code: Some("en-US".to_string()),
    template_args: {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "Alice".to_string());
        map.insert("location".to_string(), "New York".to_string());
        map
    },
    use_emojis: false,
    timezone: Some("America/New_York".to_string()),
};

let message = generate_goodbye(&options)?;
```

## Performance

- Typical execution time: <10ms
- Thread-safe: Yes (uses `Arc` for shared corpus, no mutable shared state)
- Memory usage: <10MB (includes corpus)

## Platform Support

- Rust 1.91.1+
- Platforms: Linux, macOS, Windows
- Architectures: x86_64, aarch64 (ARM64)

