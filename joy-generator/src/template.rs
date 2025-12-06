use crate::datetime::{get_current_date, get_current_time};
use crate::error::GoodbyeError;
use crate::options::CoreGoodbyeOptions;

/// Replace template variables in a message template
/// Supports: {name}, {location}, {date}, {time}
/// Defaults: name="Good Soul", location="The World", date/time from timezone
pub fn replace_template_variables(
    template: &str,
    options: &CoreGoodbyeOptions,
) -> Result<String, GoodbyeError> {
    let mut result = template.to_string();

    // Get default values
    let name = options
        .template_args
        .get("name")
        .map(|s| s.as_str())
        .unwrap_or("Good Soul");

    let location = options
        .template_args
        .get("location")
        .map(|s| s.as_str())
        .unwrap_or("The World");

    // Generate date/time if not provided
    let date = if let Some(d) = options.template_args.get("date") {
        d.clone()
    } else {
        get_current_date(&options.timezone)?
    };

    let time = if let Some(t) = options.template_args.get("time") {
        t.clone()
    } else {
        get_current_time(&options.timezone)?
    };

    // Replace template variables
    result = result.replace("{name}", name);
    result = result.replace("{location}", location);
    result = result.replace("{date}", &date);
    result = result.replace("{time}", &time);

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
}
