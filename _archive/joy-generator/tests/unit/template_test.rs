use joy_generator::{CoreGoodbyeOptions};
use joy_generator::template::replace_template_variables;

#[test]
fn test_replace_name_with_custom_value() {
    let mut options = CoreGoodbyeOptions::default();
    options.template_args.insert("name".to_string(), "Alice".to_string());
    
    let result = replace_template_variables("Hello, {name}!", &options).unwrap();
    assert_eq!(result, "Hello, Alice!");
}

#[test]
fn test_replace_name_with_default() {
    let options = CoreGoodbyeOptions::default();
    let result = replace_template_variables("Hello, {name}!", &options).unwrap();
    assert_eq!(result, "Hello, Good Soul!");
}

#[test]
fn test_replace_location_with_custom_value() {
    let mut options = CoreGoodbyeOptions::default();
    options.template_args.insert("location".to_string(), "Paris".to_string());
    
    let result = replace_template_variables("From {location}", &options).unwrap();
    assert_eq!(result, "From Paris");
}

#[test]
fn test_replace_location_with_default() {
    let options = CoreGoodbyeOptions::default();
    let result = replace_template_variables("From {location}", &options).unwrap();
    assert_eq!(result, "From The World");
}

#[test]
fn test_replace_date_with_timezone() {
    let options = CoreGoodbyeOptions::default();
    let result = replace_template_variables("Today is {date}", &options).unwrap();
    // Should be in YYYY-MM-DD format
    assert!(result.starts_with("Today is "));
    assert!(result.len() > 10);
    assert!(result.contains('-'));
}

#[test]
fn test_replace_time_with_timezone() {
    let options = CoreGoodbyeOptions::default();
    let result = replace_template_variables("Time is {time}", &options).unwrap();
    // Should be in HH:MM format
    assert!(result.starts_with("Time is "));
    assert!(result.len() > 8);
    assert!(result.contains(':'));
}

#[test]
fn test_replace_multiple_variables() {
    let mut options = CoreGoodbyeOptions::default();
    options.template_args.insert("name".to_string(), "Bob".to_string());
    options.template_args.insert("location".to_string(), "London".to_string());
    
    let result = replace_template_variables("Hello {name} from {location}", &options).unwrap();
    assert_eq!(result, "Hello Bob from London");
}

#[test]
fn test_messages_without_template_variables() {
    let options = CoreGoodbyeOptions::default();
    let result = replace_template_variables("Until we meet again", &options).unwrap();
    assert_eq!(result, "Until we meet again");
}

#[test]
fn test_custom_date_time() {
    let mut options = CoreGoodbyeOptions::default();
    options.template_args.insert("date".to_string(), "2025-01-27".to_string());
    options.template_args.insert("time".to_string(), "14:30".to_string());
    
    let result = replace_template_variables("{date} at {time}", &options).unwrap();
    assert_eq!(result, "2025-01-27 at 14:30");
}

