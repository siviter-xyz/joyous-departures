/// Strip all emoji characters from a string
/// Uses Unicode emoji properties for detection
pub fn strip_emojis(text: &str) -> String {
    text.chars().filter(|&c| !is_emoji_char(c)).collect()
}

/// Check if a string contains emojis
pub fn has_emojis(text: &str) -> bool {
    text.chars().any(|c| is_emoji_char(c))
}

/// Check if a character is an emoji
/// Uses Unicode ranges for emoji detection (simplified but comprehensive)
fn is_emoji_char(c: char) -> bool {
    let code = c as u32;

    // Main emoji ranges
    (code >= 0x1F300 && code <= 0x1F9FF) ||  // Miscellaneous Symbols and Pictographs, Emoticons
    (code >= 0x2600 && code <= 0x26FF) ||    // Miscellaneous Symbols
    (code >= 0x2700 && code <= 0x27BF) ||     // Dingbats
    (code >= 0xFE00 && code <= 0xFE0F) ||    // Variation Selectors
    (code >= 0x1F900 && code <= 0x1F9FF) ||  // Supplemental Symbols and Pictographs
    (code >= 0x1F1E0 && code <= 0x1F1FF) ||   // Regional Indicator Symbols (flags)
    (code >= 0x1FA00 && code <= 0x1FAFF) ||   // Chess symbols, etc.
    // Common emoji code points
    code == 0x200D ||  // Zero Width Joiner
    code == 0x20E3 ||  // Combining Enclosing Keycap
    code == 0x203C ||  // Double Exclamation Mark
    code == 0x2049 ||  // Exclamation Question Mark
    code == 0x2122 ||  // Trade Mark Sign
    code == 0x2139 ||  // Information Source
    (code >= 0x2194 && code <= 0x2199) ||    // Arrows
    (code >= 0x21A9 && code <= 0x21AA) ||    // Arrows
    (code >= 0x231A && code <= 0x231B) ||    // Watch, Hourglass
    code == 0x2328 ||  // Keyboard
    code == 0x23CF ||  // Eject Symbol
    (code >= 0x23E9 && code <= 0x23F3) ||    // Media controls
    (code >= 0x23F8 && code <= 0x23FA) ||    // Media controls
    code == 0x24C2 ||  // Circled Latin Capital Letter M
    (code >= 0x25AA && code <= 0x25AB) ||    // Geometric shapes
    code == 0x25B6 ||  // Play button
    code == 0x25C0 ||  // Reverse button
    (code >= 0x25FB && code <= 0x25FE) ||    // Geometric shapes
    (code >= 0x2600 && code <= 0x2604) ||    // Weather symbols
    code == 0x260E ||  // Telephone
    code == 0x2611 ||  // Ballot Box with Check
    (code >= 0x2614 && code <= 0x2615) ||    // Umbrella, Hot Beverage
    code == 0x2618 ||  // Shamrock
    code == 0x261D ||  // Index Pointing Up
    code == 0x2620 ||  // Skull and Crossbones
    (code >= 0x2622 && code <= 0x2623) ||    // Radioactive, Biohazard
    code == 0x2626 ||  // Orthodox Cross
    code == 0x262A ||  // Star and Crescent
    (code >= 0x262E && code <= 0x262F) ||     // Peace symbols
    (code >= 0x2638 && code <= 0x263A) ||    // Wheel of Dharma, White Smiling Face
    code == 0x2640 ||  // Female Sign
    code == 0x2642 ||  // Male Sign
    (code >= 0x2648 && code <= 0x2653) ||     // Zodiac symbols
    (code >= 0x2660 && code <= 0x2668) ||    // Card suits, music
    code == 0x267B ||  // Recycling Symbol
    (code >= 0x267E && code <= 0x267F) ||    // Infinity, Wheelchair
    (code >= 0x2692 && code <= 0x2697) ||    // Tools
    code == 0x2699 ||  // Gear
    (code >= 0x269B && code <= 0x269C) ||    // Atom, Fleur-de-lis
    (code >= 0x26A0 && code <= 0x26A1) ||    // Warning, High Voltage
    (code >= 0x26AA && code <= 0x26AB) ||    // White/Black Circle
    (code >= 0x26B0 && code <= 0x26B1) ||    // Coffin, Funeral Urn
    (code >= 0x26BD && code <= 0x26BE) ||    // Soccer Ball, Baseball
    (code >= 0x26C4 && code <= 0x26C5) ||     // Snowman
    code == 0x26C8 ||  // Thunder Cloud and Rain
    code == 0x26CE ||  // Ophiuchus
    code == 0x26CF ||  // Pick
    code == 0x26D1 ||  // Helmet with White Cross
    (code >= 0x26D3 && code <= 0x26D4) ||     // Chains, No Entry
    (code >= 0x26E9 && code <= 0x26EA) ||    // Shinto Shrine, Church
    (code >= 0x26F0 && code <= 0x26F5) ||    // Mountain, Beach, etc.
    (code >= 0x26F7 && code <= 0x26FA) ||    // Skier, Ice Skate, etc.
    code == 0x26FD ||  // Fuel Pump
    code == 0x2702 ||  // Scissors
    code == 0x2705 ||  // White Heavy Check Mark
    (code >= 0x2708 && code <= 0x270D) ||    // Airplane, Envelope, etc.
    code == 0x270F ||  // Pencil
    code == 0x2712 ||  // Black Nib
    code == 0x2714 ||  // Heavy Check Mark
    code == 0x2716 ||  // Heavy Multiplication X
    code == 0x271D ||  // Latin Cross
    code == 0x2721 ||  // Star of David
    code == 0x2728 ||  // Sparkles
    (code >= 0x2733 && code <= 0x2734) ||    // Eight-Pointed Star
    code == 0x2744 ||  // Snowflake
    code == 0x2747 ||  // Sparkle
    code == 0x274C ||  // Cross Mark
    code == 0x274E ||  // Negative Squared Cross Mark
    (code >= 0x2753 && code <= 0x2755) ||    // Question marks
    code == 0x2757 ||  // Heavy Exclamation Mark
    (code >= 0x2763 && code <= 0x2764) ||    // Heart
    (code >= 0x2795 && code <= 0x2797) ||    // Plus, Minus, Division
    code == 0x27A1 ||  // Rightwards Arrow
    code == 0x27B0 ||  // Curly Loop
    code == 0x27BF ||  // Double Curly Loop
    (code >= 0x2934 && code <= 0x2935) ||    // Arrows
    (code >= 0x2B05 && code <= 0x2B07) ||    // Arrows
    (code >= 0x2B1B && code <= 0x2B1C) ||    // Black/White Large Square
    code == 0x2B50 ||  // White Medium Star
    code == 0x2B55 ||  // Heavy Large Circle
    code == 0x3030 ||  // Wavy Dash
    code == 0x303D ||  // Part Alternation Mark
    (code >= 0x3297 && code <= 0x3299) // Circled Ideographs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_emojis() {
        let text = "Hello ❤️ World ✨";
        let result = strip_emojis(text);
        assert_eq!(result, "Hello  World ");
    }

    #[test]
    fn test_has_emojis() {
        assert!(has_emojis("Hello ❤️"));
        assert!(!has_emojis("Hello World"));
    }

    #[test]
    fn test_strip_no_emojis() {
        let text = "Hello World";
        let result = strip_emojis(text);
        assert_eq!(result, "Hello World");
    }
}
