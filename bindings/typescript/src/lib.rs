use joy_generator::{generate_goodbye as rust_generate_goodbye, CoreGoodbyeOptions, GoodbyeError};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Constants for fallback messages
const FALLBACK_MESSAGE_WITH_NAME: &str = "Wishing you a joyous day, {name} ❤️";
const FALLBACK_MESSAGE_WITHOUT_TEMPLATE: &str = "Wishing you a joyous day❤️";

// Maximum JSON input size to prevent DoS (1MB)
const MAX_JSON_SIZE: usize = 1_048_576;

#[wasm_bindgen]
pub fn generate_goodbye(
    language_code: String,
    template_args_json: String,
    use_emojis: bool,
    timezone: String,
) -> String {
    let mut options = CoreGoodbyeOptions {
        language_code,
        template_args: HashMap::new(),
        use_emojis,
        timezone,
    };

    // Parse template args from JSON with size limit
    if !template_args_json.is_empty() {
        // Check size before parsing to prevent DoS
        if template_args_json.len() > MAX_JSON_SIZE {
            // Return fallback if input too large
            let default_name = "Good Soul".to_string();
            let name = options.template_args.get("name").unwrap_or(&default_name);
            return FALLBACK_MESSAGE_WITH_NAME.replace("{name}", name);
        }

        if let Ok(args) = serde_json::from_str::<HashMap<String, String>>(&template_args_json) {
            // Validate and truncate template args
            let mut validated_args = HashMap::new();
            for (key, value) in args {
                // Truncate values based on key
                let max_len = match key.as_str() {
                    "name" => 50,
                    "location" => 100,
                    "date" => 20,
                    "time" => 10,
                    _ => 200, // Default max for unknown keys
                };
                let truncated = if value.len() > max_len {
                    value.chars().take(max_len).collect()
                } else {
                    value
                };
                validated_args.insert(key, truncated);
            }
            options.template_args = validated_args;
        }
    }

    // Call Rust core function with proper error handling
    match rust_generate_goodbye(&options) {
        Ok(result) => result,
        Err(GoodbyeError::CorpusLoadError(_)) => {
            // Return fallback message with name
            let default_name = "Good Soul".to_string();
            let name = options.template_args.get("name").unwrap_or(&default_name);
            FALLBACK_MESSAGE_WITH_NAME.replace("{name}", name)
        }
        Err(GoodbyeError::TemplateVariableError(_)) => {
            // Return fallback without templates
            FALLBACK_MESSAGE_WITHOUT_TEMPLATE.to_string()
        }
        Err(GoodbyeError::InvalidLanguageCodeError(_)) => {
            // Fallback to en-GB
            options.language_code = "en-GB".to_string();
            rust_generate_goodbye(&options).unwrap_or_else(|_| {
                let default_name = "Good Soul".to_string();
                let name = options.template_args.get("name").unwrap_or(&default_name);
                FALLBACK_MESSAGE_WITH_NAME.replace("{name}", name)
            })
        }
        Err(GoodbyeError::InvalidTimezoneError(_)) => {
            // Fallback to Europe/London
            options.timezone = "Europe/London".to_string();
            rust_generate_goodbye(&options).unwrap_or_else(|_| {
                let default_name = "Good Soul".to_string();
                let name = options.template_args.get("name").unwrap_or(&default_name);
                FALLBACK_MESSAGE_WITH_NAME.replace("{name}", name)
            })
        }
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    // Initialize WASM module if needed
}
