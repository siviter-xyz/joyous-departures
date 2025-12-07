use pyo3::prelude::*;
use pyo3::types::PyDict;

use joy_generator::{generate_goodbye as rust_generate_goodbye, CoreGoodbyeOptions, GoodbyeError};

// Constants for fallback messages
const FALLBACK_MESSAGE_WITH_NAME: &str = "Wishing you a joyous day, {name} ❤️";
const FALLBACK_MESSAGE_WITHOUT_TEMPLATE: &str = "Wishing you a joyous day❤️";

/// Generate a warm, heartfelt goodbye message
///
/// Args:
///     language_code: Optional ISO 639-1 language code (default: "en-GB")
///     template_args: Optional dict with template variables (name, location, date, time)
///     use_emojis: Optional bool to include emojis (default: True)
///     timezone: Optional IANA timezone identifier (default: "Europe/London")
///
/// Returns:
///     str: Generated goodbye message
///
/// Raises:
///     ValueError: If language code or timezone is invalid
#[pyfunction]
#[pyo3(signature = (
    *,
    language_code = None,
    template_args = None,
    use_emojis = true,
    timezone = "Europe/London"
))]
fn generate_goodbye(
    _py: Python,
    language_code: Option<&str>,
    template_args: Option<Bound<'_, PyDict>>,
    use_emojis: bool,
    timezone: &str,
) -> PyResult<String> {
    // Validate and prepare options
    let mut options = CoreGoodbyeOptions::default();

    // Set language code (validate format: ISO 639-1 with optional region)
    if let Some(lang) = language_code {
        if !is_valid_language_code(lang) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid language code: {}. Must be ISO 639-1 format (e.g., 'en-GB', 'en-US')",
                lang
            )));
        }
        options.language_code = lang.to_string();
    }

    // Validate timezone (IANA format)
    if !is_valid_timezone(timezone) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Invalid timezone: {}. Must be IANA timezone identifier (e.g., 'Europe/London', 'America/New_York')", timezone)
        ));
    }
    options.timezone = timezone.to_string();

    // Set template args with validation and truncation
    if let Some(template_dict) = template_args {
        for (key, value) in template_dict.iter() {
            let key_str = key.extract::<String>()?;
            let value_str = value.extract::<String>()?;

            // Truncate values based on key to prevent excessive input
            let max_len = match key_str.as_str() {
                "name" => 50,
                "location" => 100,
                "date" => 20,
                "time" => 10,
                _ => 200, // Default max for unknown keys
            };

            let truncated = if value_str.len() > max_len {
                value_str.chars().take(max_len).collect()
            } else {
                value_str
            };

            options.template_args.insert(key_str, truncated);
        }
    }

    options.use_emojis = use_emojis;

    // Call Rust core (handle errors with fallbacks)
    match rust_generate_goodbye(&options) {
        Ok(result) => Ok(result),
        Err(GoodbyeError::CorpusLoadError(_)) => {
            // Return fallback message
            let default_name = "Good Soul".to_string();
            let name = options.template_args.get("name").unwrap_or(&default_name);
            let result = FALLBACK_MESSAGE_WITH_NAME.replace("{name}", name);
            Ok(result)
        }
        Err(GoodbyeError::TemplateVariableError(_)) => {
            // Return fallback without templates
            Ok(FALLBACK_MESSAGE_WITHOUT_TEMPLATE.to_string())
        }
        Err(GoodbyeError::InvalidLanguageCodeError(_)) => {
            // Fallback to en-GB
            options.language_code = "en-GB".to_string();
            rust_generate_goodbye(&options).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to generate message: {}",
                    e
                ))
            })
        }
        Err(GoodbyeError::InvalidTimezoneError(_)) => {
            // Fallback to Europe/London
            options.timezone = "Europe/London".to_string();
            rust_generate_goodbye(&options).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to generate message: {}",
                    e
                ))
            })
        }
    }
}

/// Validate ISO 639-1 language code format
fn is_valid_language_code(code: &str) -> bool {
    // Basic validation: 2-5 characters, optional hyphen and region
    // Examples: "en", "en-GB", "fr", "fr-FR"
    let parts: Vec<&str> = code.split('-').collect();
    if parts.is_empty() || parts.len() > 2 {
        return false;
    }
    parts[0].len() == 2
        && parts[0].chars().all(|c| c.is_ascii_lowercase())
        && (parts.len() == 1
            || (parts[1].len() == 2 && parts[1].chars().all(|c| c.is_ascii_uppercase())))
}

/// Validate IANA timezone identifier
fn is_valid_timezone(tz: &str) -> bool {
    // Basic validation: should contain a slash (e.g., "Europe/London")
    // More thorough validation would use chrono-tz, but this is handled in Rust core
    tz.contains('/') && !tz.is_empty()
}

/// Python module definition
#[pymodule]
fn _joyous_departures(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_goodbye, m)?)?;
    m.add("__version__", "0.1.0")?;
    Ok(())
}
