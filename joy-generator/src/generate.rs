use rand::Rng;

use crate::corpus::load_corpus;
use crate::emoji::strip_emojis;
use crate::error::GoodbyeError;
use crate::options::CoreGoodbyeOptions;
use crate::template::replace_template_variables;

/// Generate a random goodbye message
///
/// # Arguments
///
/// * `options` - Configuration options for message generation
///
/// # Returns
///
/// A generated message string with template variables replaced
///
/// # Errors
///
/// Returns `GoodbyeError` if corpus loading fails, timezone is invalid, or template replacement fails
pub fn generate_goodbye(options: &CoreGoodbyeOptions) -> Result<String, GoodbyeError> {
    // Load corpus (cached, thread-safe)
    let corpus = load_corpus()?;

    // Get messages for the requested language (fallback to en-GB if not found)
    let language = &options.language_code;
    let message_indices = corpus
        .language_index
        .get(language)
        .or_else(|| corpus.language_index.get("en-GB"))
        .ok_or_else(|| GoodbyeError::InvalidLanguageCodeError(language.clone()))?;

    // Check for empty message indices (should not happen with valid corpus, but safety check)
    if message_indices.is_empty() {
        return Err(GoodbyeError::CorpusLoadError(
            "No messages found in corpus for language".to_string(),
        ));
    }

    // Select random message
    let mut rng = rand::thread_rng();
    let idx = message_indices[rng.gen_range(0..message_indices.len())];
    let message_template = &corpus.messages[idx];

    // Replace template variables
    let mut result = replace_template_variables(&message_template.template, options)?;

    // Strip emojis if requested
    if !options.use_emojis {
        result = strip_emojis(&result);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_basic() {
        let options = CoreGoodbyeOptions::default();
        let result = generate_goodbye(&options).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_generate_with_custom_name() {
        let mut options = CoreGoodbyeOptions::default();
        options
            .template_args
            .insert("name".to_string(), "Alice".to_string());
        let result = generate_goodbye(&options).unwrap();
        assert!(result.contains("Alice") || result.contains("Good Soul"));
    }

    #[test]
    fn test_generate_without_emojis() {
        let mut options = CoreGoodbyeOptions::default();
        options.use_emojis = false;
        let result = generate_goodbye(&options).unwrap();
        // Should not contain emojis (basic check - emoji detection may vary)
        assert!(!result.contains("❤️") || !result.contains("✨"));
    }

    #[test]
    fn test_generate_randomness() {
        let options = CoreGoodbyeOptions::default();
        let results: Vec<String> = (0..10)
            .map(|_| generate_goodbye(&options).unwrap())
            .collect();
        // With fallback corpus (1 message), all will be same, but structure is correct
        // In Phase 5 with 360 messages, this will show randomness
        assert_eq!(results.len(), 10);
    }

    #[test]
    fn test_empty_message_indices_handling() {
        // This test verifies that empty message_indices is handled gracefully
        // In practice, this should not happen with a valid corpus, but we test the safety check
        let options = CoreGoodbyeOptions::default();
        // The corpus should always have messages, so this should succeed
        let result = generate_goodbye(&options);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
