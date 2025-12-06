use joy_generator::{generate_goodbye as rust_generate_goodbye, CoreGoodbyeOptions};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

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

    // Parse template args from JSON
    if !template_args_json.is_empty() {
        if let Ok(args) = serde_json::from_str::<HashMap<String, String>>(&template_args_json) {
            options.template_args = args;
        }
    }

    // Call Rust core function
    match rust_generate_goodbye(&options) {
        Ok(result) => result,
        Err(_) => {
            // Return fallback message on error
            let default_name = "Good Soul".to_string();
            let name = options.template_args.get("name").unwrap_or(&default_name);
            format!("Wishing you a joyous day, {} ❤️", name)
        }
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    // Initialize WASM module if needed
}
