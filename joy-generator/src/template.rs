use crate::datetime::{get_current_date, get_current_time};
use crate::error::GoodbyeError;
use crate::options::CoreGoodbyeOptions;

/// Sanitize template variable value to prevent injection
/// Removes control characters (except whitespace) and limits length
fn sanitize_template_value(value: &str, max_length: usize) -> String {
    value
        .chars()
        // Keep characters that are either:
        // - Not control characters, OR
        // - Control characters that are whitespace (e.g., newline, tab)
        .filter(|c| !c.is_control() || (c.is_control() && c.is_whitespace()))
        .take(max_length)
        .collect()
}

/// Replace template variables in a message template
/// Supports: {name}, {location}, {date}, {time}
/// Defaults: name="Good Soul", location="The World", date/time from timezone
///
/// Variables are replaced in a specific order to prevent nested replacement issues:
/// 1. date/time first (unlikely to contain template syntax)
/// 2. location (may contain special chars but not template syntax)
/// 3. name last (most likely to contain user input)
pub fn replace_template_variables(
    template: &str,
    options: &CoreGoodbyeOptions,
) -> Result<String, GoodbyeError> {
    let mut result = template.to_string();

    // Get default values and sanitize
    let name = options
        .template_args
        .get("name")
        .map(|s| sanitize_template_value(s, 50))
        .unwrap_or_else(|| "Good Soul".to_string());

    let location = options
        .template_args
        .get("location")
        .map(|s| sanitize_template_value(s, 100))
        .unwrap_or_else(|| "The World".to_string());

    // Generate date/time if not provided
    let date = if let Some(d) = options.template_args.get("date") {
        sanitize_template_value(d, 20) // Date format: YYYY-MM-DD (10 chars, but allow margin)
    } else {
        get_current_date(&options.timezone)?
    };

    let time = if let Some(t) = options.template_args.get("time") {
        sanitize_template_value(t, 10) // Time format: HH:MM (5 chars, but allow margin)
    } else {
        get_current_time(&options.timezone)?
    };

    // Replace template variables in order to prevent nested replacement
    // Replace date/time first (least likely to contain template syntax)
    result = result.replace("{date}", &date);
    result = result.replace("{time}", &time);
    // Then location
    result = result.replace("{location}", &location);
    // Finally name (most likely to contain user input)
    result = result.replace("{name}", &name);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_name() {
        let mut options = CoreGoodbyeOptions::default();
        options
            .template_args
            .insert("name".to_string(), "Alice".to_string());

        let result = replace_template_variables("Hello, {name}!", &options).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_replace_default_name() {
        let options = CoreGoodbyeOptions::default();
        let result = replace_template_variables("Hello, {name}!", &options).unwrap();
        assert_eq!(result, "Hello, Good Soul!");
    }

    #[test]
    fn test_replace_location() {
        let mut options = CoreGoodbyeOptions::default();
        options
            .template_args
            .insert("location".to_string(), "Paris".to_string());

        let result = replace_template_variables("From {location}", &options).unwrap();
        assert_eq!(result, "From Paris");
    }

    #[test]
    fn test_replace_date_time() {
        let options = CoreGoodbyeOptions::default();
        let result = replace_template_variables("{date} {time}", &options).unwrap();
        assert!(result.contains('-') && result.contains(':')); // Date and time format
    }

    #[test]
    fn test_no_template_variables() {
        let options = CoreGoodbyeOptions::default();
        let result = replace_template_variables("No variables here", &options).unwrap();
        assert_eq!(result, "No variables here");
    }

    #[test]
    fn test_template_injection_prevention() {
        // Test that nested template variables in name don't cause double replacement
        let mut options = CoreGoodbyeOptions::default();
        options
            .template_args
            .insert("name".to_string(), "{location}".to_string());
        options
            .template_args
            .insert("location".to_string(), "Paris".to_string());

        // Name should be replaced with literal "{location}", not with "Paris"
        let result = replace_template_variables("Hello, {name}!", &options).unwrap();
        assert_eq!(result, "Hello, {location}!");
    }

    #[test]
    fn test_sanitize_control_characters() {
        // Test that control characters are removed
        let mut options = CoreGoodbyeOptions::default();
        options
            .template_args
            .insert("name".to_string(), "Alice\x00\x01\x02".to_string());

        let result = replace_template_variables("Hello, {name}!", &options).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_truncate_long_values() {
        // Test that long values are truncated
        let mut options = CoreGoodbyeOptions::default();
        let long_name = "A".repeat(100);
        options
            .template_args
            .insert("name".to_string(), long_name.clone());

        let result = replace_template_variables("Hello, {name}!", &options).unwrap();
        // Should be truncated to 50 chars
        assert!(result.len() < long_name.len());
        assert!(result.contains("Hello,"));
    }
}
