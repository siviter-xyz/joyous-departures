use joy_generator::emoji::{has_emojis, strip_emojis};

#[test]
fn test_emoji_detection() {
    assert!(has_emojis("Hello ❤️"));
    assert!(has_emojis("Wishing you well ✨"));
    assert!(has_emojis("Take care 🌻"));
    assert!(!has_emojis("Hello World"));
    assert!(!has_emojis("Until we meet again"));
}

#[test]
fn test_emoji_stripping() {
    let text = "Hello ❤️ World ✨";
    let result = strip_emojis(text);
    assert_eq!(result, "Hello  World ");
    assert!(!has_emojis(&result));
}

#[test]
fn test_emoji_preservation_when_not_stripped() {
    let text = "Hello ❤️ World ✨";
    // When not stripping, emojis should remain
    assert!(has_emojis(text));
}

#[test]
fn test_strip_no_emojis() {
    let text = "Hello World";
    let result = strip_emojis(text);
    assert_eq!(result, "Hello World");
}

#[test]
fn test_strip_multiple_emojis() {
    let text = "Wishing you ❤️ joy ✨ and 🌻 peace 🕊️";
    let result = strip_emojis(text);
    assert!(!has_emojis(&result));
    assert!(result.contains("Wishing you"));
    assert!(result.contains("joy"));
    assert!(result.contains("peace"));
}

#[test]
fn test_emoji_only_string() {
    let text = "❤️✨🌻";
    let result = strip_emojis(text);
    assert_eq!(result, "");
}


