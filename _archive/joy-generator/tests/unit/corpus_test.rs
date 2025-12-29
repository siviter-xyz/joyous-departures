use joy_generator::corpus::{load_corpus, MessageCorpus};

#[test]
fn test_corpus_decompression() {
    // Corpus should load successfully (even if using fallback)
    let corpus = load_corpus().unwrap();
    assert!(!corpus.messages.is_empty());
}

#[test]
fn test_corpus_initialization() {
    let corpus = load_corpus().unwrap();
    // Should have at least one message (fallback if corpus empty)
    assert!(corpus.messages.len() > 0);
}

#[test]
fn test_message_count() {
    let corpus = load_corpus().unwrap();
    // With fallback corpus, we have 1 message
    // In Phase 5, this will be 360 messages
    assert!(corpus.messages.len() >= 1);
    
    // Verify message structure
    for msg in &corpus.messages {
        assert!(!msg.template.is_empty());
        assert!(!msg.language_code.is_empty());
    }
}

#[test]
fn test_language_index() {
    let corpus = load_corpus().unwrap();
    // Should have en-GB in index (fallback corpus)
    assert!(corpus.language_index.contains_key("en-GB"));
}

#[test]
fn test_corpus_thread_safety() {
    use std::sync::Arc;
    use std::thread;
    
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let corpus1 = load_corpus().unwrap();
                let corpus2 = load_corpus().unwrap();
                // Should be the same Arc (same pointer)
                assert!(Arc::ptr_eq(&corpus1, &corpus2));
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}


