use std::sync::Arc;
use std::sync::Once;

use crate::emoji::has_emojis;
use crate::error::GoodbyeError;

/// A single message template
#[derive(Debug, Clone)]
pub struct MessageTemplate {
    /// The template string with optional variables like {name}, {location}, {date}, {time}
    pub template: String,
    /// Language code (e.g., "en-GB")
    pub language_code: String,
    /// Whether this message contains emojis
    pub has_emojis: bool,
}

/// Collection of message templates
#[derive(Debug, Clone)]
pub struct MessageCorpus {
    /// All messages in the corpus
    pub messages: Vec<MessageTemplate>,
    /// Index by language code for fast lookup
    pub language_index: std::collections::HashMap<String, Vec<usize>>,
}

// Embedded corpus data (loaded at build time)
// The corpus is embedded directly as text - it's small enough that compression isn't necessary
static CORPUS_TEXT: &str = include_str!("../../corpus/en-GB.txt");

/// Load and cache the message corpus
/// This is called once on first use, then cached in Arc for thread-safety
pub fn load_corpus() -> Result<Arc<MessageCorpus>, GoodbyeError> {
    static INIT: Once = Once::new();
    static mut CORPUS: Option<Arc<MessageCorpus>> = None;

    unsafe {
        INIT.call_once(|| {
            let corpus = if CORPUS_TEXT.is_empty() {
                // Fallback if corpus is empty
                create_fallback_corpus()
            } else {
                // Parse corpus (one message per line)
                let messages: Vec<MessageTemplate> = CORPUS_TEXT
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| MessageTemplate {
                        template: line.trim().to_string(),
                        language_code: "en-GB".to_string(),
                        has_emojis: has_emojis(line),
                    })
                    .collect();

                if messages.is_empty() {
                    let fallback = create_fallback_corpus();
                    CORPUS = Some(Arc::new(fallback));
                    return;
                }

                // Build language index
                let mut language_index: std::collections::HashMap<String, Vec<usize>> =
                    std::collections::HashMap::new();
                for (idx, msg) in messages.iter().enumerate() {
                    language_index
                        .entry(msg.language_code.clone())
                        .or_insert_with(Vec::new)
                        .push(idx);
                }

                MessageCorpus {
                    messages,
                    language_index,
                }
            };

            let corpus_arc = Arc::new(corpus);
            CORPUS = Some(corpus_arc);
        });

        // SAFETY: INIT.call_once ensures CORPUS is initialized
        unsafe {
            Ok(CORPUS.as_ref().unwrap_unchecked().clone())
        }
    }
}

/// Create a fallback corpus with a single default message
/// Used when corpus file is empty or corrupted
fn create_fallback_corpus() -> MessageCorpus {
    let fallback = MessageTemplate {
        template: "Wishing you a joyous day, {name} ❤️".to_string(),
        language_code: "en-GB".to_string(),
        has_emojis: true,
    };

    let mut language_index = std::collections::HashMap::new();
    language_index.insert("en-GB".to_string(), vec![0]);

    MessageCorpus {
        messages: vec![fallback],
        language_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_corpus() {
        let corpus = load_corpus().unwrap();
        assert!(!corpus.messages.is_empty());
    }

    #[test]
    fn test_fallback_corpus() {
        let fallback = create_fallback_corpus();
        assert_eq!(fallback.messages.len(), 1);
        assert!(fallback.messages[0].template.contains("{name}"));
    }

    #[test]
    fn test_corpus_caching() {
        let corpus1 = load_corpus().unwrap();
        let corpus2 = load_corpus().unwrap();
        // Should be the same Arc (same pointer)
        assert!(Arc::ptr_eq(&corpus1, &corpus2));
    }
}
