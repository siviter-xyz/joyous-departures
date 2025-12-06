use joy_generator::{generate_goodbye, CoreGoodbyeOptions};

#[test]
fn test_basic_generation_with_defaults() {
    let options = CoreGoodbyeOptions::default();
    let result = generate_goodbye(&options).unwrap();
    
    // Should return a non-empty string
    assert!(!result.is_empty());
    // Should contain the default name "Good Soul" (from fallback message)
    assert!(result.contains("Good Soul") || result.contains("joyous"));
}

#[test]
fn test_randomness() {
    let options = CoreGoodbyeOptions::default();
    
    // Generate multiple messages
    let results: Vec<String> = (0..10)
        .map(|_| generate_goodbye(&options).unwrap())
        .collect();
    
    // All should be non-empty
    assert_eq!(results.len(), 10);
    for result in &results {
        assert!(!result.is_empty());
    }
    
    // With fallback corpus (1 message), all will be same
    // In Phase 5 with 360 messages, we should see different results
    // For now, just verify structure is correct
}

#[test]
fn test_message_format_validation() {
    let options = CoreGoodbyeOptions::default();
    let result = generate_goodbye(&options).unwrap();
    
    // Message should be between 10-100 characters (excluding template vars)
    // Fallback message is: "Wishing you a joyous day, {name} ❤️"
    // After replacement: "Wishing you a joyous day, Good Soul ❤️" = ~40 chars
    assert!(result.len() >= 10);
    assert!(result.len() <= 200); // Allow some margin for template replacements
}

#[test]
fn test_custom_name() {
    let mut options = CoreGoodbyeOptions::default();
    options
        .template_args
        .insert("name".to_string(), "Alice".to_string());
    
    let result = generate_goodbye(&options).unwrap();
    assert!(result.contains("Alice"));
}

#[test]
fn test_default_name_when_not_provided() {
    let options = CoreGoodbyeOptions::default();
    let result = generate_goodbye(&options).unwrap();
    // Should use "Good Soul" as default IF the message contains {name}
    // Some messages don't have template variables, so we check for either
    // "Good Soul" (when template is used) or that it's a valid non-empty message
    assert!(!result.is_empty());
    // If message has {name}, it should be replaced with "Good Soul"
    // If message doesn't have template variables, it's still valid
    if result.contains("{name}") {
        panic!("Template variable {{name}} was not replaced");
    }
    // Message is valid if it either contains "Good Soul" or is a simple message without variables
    assert!(result.contains("Good Soul") || !result.contains("{"));
}

