use std::sync::Arc;
use std::sync::Once;

use lz4::Decoder;
use std::io::Read;

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

// Embedded compressed corpus data (set at build time)
// This will be populated by a build script that compresses corpus/en-GB.txt
// For now, we use an empty slice - corpus will be populated in Phase 5
// When corpus is ready, uncomment and use: include_bytes!("../../corpus/en-GB.txt.lz4");
static COMPRESSED_CORPUS: &[u8] = &[];

/// Load and cache the message corpus
/// This is called once on first use, then cached in Arc for thread-safety
pub fn load_corpus() -> Result<Arc<MessageCorpus>, GoodbyeError> {
    static INIT: Once = Once::new();
    static mut CORPUS: Option<Arc<MessageCorpus>> = None;

    unsafe {
        INIT.call_once(|| {
            let corpus = if COMPRESSED_CORPUS.is_empty() {
                // For now, if corpus is empty, return fallback
                // In Phase 5, we'll populate the corpus file
                create_fallback_corpus()
            } else {
                // Decompress corpus
                let mut decoder = match Decoder::new(COMPRESSED_CORPUS) {
                    Ok(d) => d,
                    Err(_) => {
                        CORPUS = Some(Arc::new(create_fallback_corpus()));
                        return;
                    }
                };

                let mut decompressed = Vec::new();
                if decoder.read_to_end(&mut decompressed).is_err() {
                    CORPUS = Some(Arc::new(create_fallback_corpus()));
                    return;
                }

                // Parse corpus (one message per line)
                let text = match String::from_utf8(decompressed) {
                    Ok(t) => t,
                    Err(_) => {
                        CORPUS = Some(Arc::new(create_fallback_corpus()));
                        return;
                    }
                };

                let messages: Vec<MessageTemplate> = text
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| MessageTemplate {
                        template: line.trim().to_string(),
                        language_code: "en-GB".to_string(),
                        has_emojis: has_emojis(line),
                    })
                    .collect();

                if messages.is_empty() {
                    CORPUS = Some(Arc::new(create_fallback_corpus()));
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

            CORPUS = Some(Arc::new(corpus));
        });

        Ok(CORPUS.as_ref().unwrap().clone())
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
