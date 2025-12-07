/// Strip all emoji characters from a string
/// Uses Unicode emoji properties for detection
pub fn strip_emojis(text: &str) -> String {
    text.chars().filter(|&c| !is_emoji_char(c)).collect()
}

/// Check if a string contains emojis
pub fn has_emojis(text: &str) -> bool {
    text.chars().any(is_emoji_char)
}

/// Check if a character is an emoji
/// Uses Unicode ranges for emoji detection (simplified but comprehensive)
fn is_emoji_char(c: char) -> bool {
    let code = c as u32;

    // Main emoji ranges
    (0x1F300..=0x1F9FF).contains(&code) ||  // Miscellaneous Symbols and Pictographs, Emoticons
    (0x2600..=0x26FF).contains(&code) ||    // Miscellaneous Symbols
    (0x2700..=0x27BF).contains(&code) ||     // Dingbats
    (0xFE00..=0xFE0F).contains(&code) ||    // Variation Selectors
    (0x1F900..=0x1F9FF).contains(&code) ||  // Supplemental Symbols and Pictographs
    (0x1F1E0..=0x1F1FF).contains(&code) ||   // Regional Indicator Symbols (flags)
    (0x1FA00..=0x1FAFF).contains(&code) ||   // Chess symbols, etc.
    // Common emoji code points
    code == 0x200D ||  // Zero Width Joiner
    code == 0x20E3 ||  // Combining Enclosing Keycap
    code == 0x203C ||  // Double Exclamation Mark
    code == 0x2049 ||  // Exclamation Question Mark
    code == 0x2122 ||  // Trade Mark Sign
    code == 0x2139 ||  // Information Source
    (0x2194..=0x2199).contains(&code) ||    // Arrows
    (0x21A9..=0x21AA).contains(&code) ||    // Arrows
    (0x231A..=0x231B).contains(&code) ||    // Watch, Hourglass
    code == 0x2328 ||  // Keyboard
    code == 0x23CF ||  // Eject Symbol
    (0x23E9..=0x23F3).contains(&code) ||    // Media controls
    (0x23F8..=0x23FA).contains(&code) ||    // Media controls
    code == 0x24C2 ||  // Circled Latin Capital Letter M
    (0x25AA..=0x25AB).contains(&code) ||    // Geometric shapes
    code == 0x25B6 ||  // Play button
    code == 0x25C0 ||  // Reverse button
    (0x25FB..=0x25FE).contains(&code) ||    // Geometric shapes
    (0x2600..=0x2604).contains(&code) ||    // Weather symbols
    code == 0x260E ||  // Telephone
    code == 0x2611 ||  // Ballot Box with Check
    (0x2614..=0x2615).contains(&code) ||    // Umbrella, Hot Beverage
    code == 0x2618 ||  // Shamrock
    code == 0x261D ||  // Index Pointing Up
    code == 0x2620 ||  // Skull and Crossbones
    (0x2622..=0x2623).contains(&code) ||    // Radioactive, Biohazard
    code == 0x2626 ||  // Orthodox Cross
    code == 0x262A ||  // Star and Crescent
    (0x262E..=0x262F).contains(&code) ||     // Peace symbols
    (0x2638..=0x263A).contains(&code) ||    // Wheel of Dharma, White Smiling Face
    code == 0x2640 ||  // Female Sign
    code == 0x2642 ||  // Male Sign
    (0x2648..=0x2653).contains(&code) ||     // Zodiac symbols
    (0x2660..=0x2668).contains(&code) ||    // Card suits, music
    code == 0x267B ||  // Recycling Symbol
    (0x267E..=0x267F).contains(&code) ||    // Infinity, Wheelchair
    (0x2692..=0x2697).contains(&code) ||    // Tools
    code == 0x2699 ||  // Gear
    (0x269B..=0x269C).contains(&code) ||    // Atom, Fleur-de-lis
    (0x26A0..=0x26A1).contains(&code) ||    // Warning, High Voltage
    (0x26AA..=0x26AB).contains(&code) ||    // White/Black Circle
    (0x26B0..=0x26B1).contains(&code) ||    // Coffin, Funeral Urn
    (0x26BD..=0x26BE).contains(&code) ||    // Soccer Ball, Baseball
    (0x26C4..=0x26C5).contains(&code) ||     // Snowman
    code == 0x26C8 ||  // Thunder Cloud and Rain
    code == 0x26CE ||  // Ophiuchus
    code == 0x26CF ||  // Pick
    code == 0x26D1 ||  // Helmet with White Cross
    (0x26D3..=0x26D4).contains(&code) ||     // Chains, No Entry
    (0x26E9..=0x26EA).contains(&code) ||    // Shinto Shrine, Church
    (0x26F0..=0x26F5).contains(&code) ||    // Mountain, Beach, etc.
    (0x26F7..=0x26FA).contains(&code) ||    // Skier, Ice Skate, etc.
    code == 0x26FD ||  // Fuel Pump
    code == 0x2702 ||  // Scissors
    code == 0x2705 ||  // White Heavy Check Mark
    (0x2708..=0x270D).contains(&code) ||    // Airplane, Envelope, etc.
    code == 0x270F ||  // Pencil
    code == 0x2712 ||  // Black Nib
    code == 0x2714 ||  // Heavy Check Mark
    code == 0x2716 ||  // Heavy Multiplication X
    code == 0x271D ||  // Latin Cross
    code == 0x2721 ||  // Star of David
    code == 0x2728 ||  // Sparkles
    (0x2733..=0x2734).contains(&code) ||    // Eight-Pointed Star
    code == 0x2744 ||  // Snowflake
    code == 0x2747 ||  // Sparkle
    code == 0x274C ||  // Cross Mark
    code == 0x274E ||  // Negative Squared Cross Mark
    (0x2753..=0x2755).contains(&code) ||    // Question marks
    code == 0x2757 ||  // Heavy Exclamation Mark
    (0x2763..=0x2764).contains(&code) ||    // Heart
    (0x2795..=0x2797).contains(&code) ||    // Plus, Minus, Division
    code == 0x27A1 ||  // Rightwards Arrow
    code == 0x27B0 ||  // Curly Loop
    code == 0x27BF ||  // Double Curly Loop
    (0x2934..=0x2935).contains(&code) ||    // Arrows
    (0x2B05..=0x2B07).contains(&code) ||    // Arrows
    (0x2B1B..=0x2B1C).contains(&code) ||    // Black/White Large Square
    code == 0x2B50 ||  // White Medium Star
    code == 0x2B55 ||  // Heavy Large Circle
    code == 0x3030 ||  // Wavy Dash
    code == 0x303D ||  // Part Alternation Mark
    (0x3297..=0x3299).contains(&code) // Circled Ideographs
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
