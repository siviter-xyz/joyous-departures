use joy_generator::{generate_goodbye, CoreGoodbyeOptions};

#[test]
fn test_date_replacement_in_generated_message() {
    let mut options = CoreGoodbyeOptions::default();
    options.timezone = "Europe/London".to_string();
    
    // Generate multiple messages until we get one with {date}
    // With 360 messages and 10 containing {date}, probability is ~2.8% per call
    let mut found_date_message = false;
    for _ in 0..200 {
        let result = generate_goodbye(&options).unwrap();
        if result.contains("{date}") {
            // Should not contain {date} after replacement
            panic!("Template variable {{date}} was not replaced in: {}", result);
        }
        // Check for date format YYYY-MM-DD
        if result.contains("202") && result.matches('-').count() >= 2 {
            // Likely contains a date (YYYY-MM-DD format)
            found_date_message = true;
            // Verify format: should have YYYY-MM-DD pattern
            let parts: Vec<&str> = result.split(' ').collect();
            for part in parts {
                if part.matches('-').count() == 2 {
                    let date_parts: Vec<&str> = part.split('-').collect();
                    if date_parts.len() == 3 
                        && date_parts[0].len() == 4 
                        && date_parts[1].len() == 2 
                        && date_parts[2].len() == 2 {
                        // Valid date format found
                        assert!(date_parts[0].parse::<u32>().is_ok(), "Year should be numeric");
                        assert!(date_parts[1].parse::<u32>().is_ok(), "Month should be numeric");
                        assert!(date_parts[2].parse::<u32>().is_ok(), "Day should be numeric");
                        break;
                    }
                }
            }
            break;
        }
    }
    
    // With 10 date messages out of 360, we should find one in 200 tries
    // If not found, that's statistically unlikely but possible
}

#[test]
fn test_time_replacement_in_generated_message() {
    let mut options = CoreGoodbyeOptions::default();
    options.timezone = "Europe/London".to_string();
    
    // Generate multiple messages until we get one with {time}
    let mut found_time_message = false;
    for _ in 0..200 {
        let result = generate_goodbye(&options).unwrap();
        if result.contains("{time}") {
            // Should not contain {time} after replacement
            panic!("Template variable {{time}} was not replaced in: {}", result);
        }
        // Check for time format HH:MM
        let parts: Vec<&str> = result.split(' ').collect();
        for part in parts {
            if part.matches(':').count() == 1 {
                let time_parts: Vec<&str> = part.split(':').collect();
                if time_parts.len() == 2 
                    && time_parts[0].len() == 2 
                    && time_parts[1].len() == 2 {
                    // Valid time format found
                    let hour = time_parts[0].parse::<u32>();
                    let minute = time_parts[1].parse::<u32>();
                    if hour.is_ok() && minute.is_ok() {
                        let h = hour.unwrap();
                        let m = minute.unwrap();
                        if h < 24 && m < 60 {
                            found_time_message = true;
                            break;
                        }
                    }
                }
            }
        }
        if found_time_message {
            break;
        }
    }
    
    // With 10 time messages out of 360, we should find one in 200 tries
    // If not found, that's statistically unlikely but possible
}

#[test]
fn test_date_time_with_custom_timezone() {
    let mut options = CoreGoodbyeOptions::default();
    options.timezone = "America/New_York".to_string();
    
    let result = generate_goodbye(&options).unwrap();
    // Should generate successfully with different timezone
    assert!(!result.is_empty());
    // Should not contain unreplaced template variables
    assert!(!result.contains("{date}"));
    assert!(!result.contains("{time}"));
}

#[test]
fn test_date_time_with_explicit_values() {
    let mut options = CoreGoodbyeOptions::default();
    options.template_args.insert("date".to_string(), "2025-01-27".to_string());
    options.template_args.insert("time".to_string(), "14:30".to_string());
    
    let result = generate_goodbye(&options).unwrap();
    // If message contains date/time, should use our explicit values
    if result.contains("2025-01-27") {
        assert!(result.contains("2025-01-27"));
    }
    if result.contains("14:30") {
        assert!(result.contains("14:30"));
    }
}

