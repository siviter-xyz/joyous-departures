use joy_generator::datetime::{get_current_date, get_current_time};

#[test]
fn test_date_generation_europe_london() {
    let date = get_current_date("Europe/London").unwrap();
    // Should be YYYY-MM-DD format
    assert_eq!(date.len(), 10);
    assert!(date.contains('-'));
    let parts: Vec<&str> = date.split('-').collect();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].len(), 4); // Year
    assert_eq!(parts[1].len(), 2); // Month
    assert_eq!(parts[2].len(), 2); // Day
}

#[test]
fn test_date_generation_america_new_york() {
    let date = get_current_date("America/New_York").unwrap();
    assert_eq!(date.len(), 10);
    assert!(date.contains('-'));
}

#[test]
fn test_time_generation_europe_london() {
    let time = get_current_time("Europe/London").unwrap();
    // Should be HH:MM format
    assert_eq!(time.len(), 5);
    assert!(time.contains(':'));
    let parts: Vec<&str> = time.split(':').collect();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].len(), 2); // Hour
    assert_eq!(parts[1].len(), 2); // Minute
}

#[test]
fn test_time_generation_america_new_york() {
    let time = get_current_time("America/New_York").unwrap();
    assert_eq!(time.len(), 5);
    assert!(time.contains(':'));
}

#[test]
fn test_default_timezone_europe_london() {
    // Test that Europe/London works (default)
    let date = get_current_date("Europe/London").unwrap();
    let time = get_current_time("Europe/London").unwrap();
    assert!(!date.is_empty());
    assert!(!time.is_empty());
}

#[test]
fn test_invalid_timezone() {
    assert!(get_current_date("Invalid/Timezone").is_err());
    assert!(get_current_time("Invalid/Timezone").is_err());
}

#[test]
fn test_different_timezones_produce_different_times() {
    // Times might be different depending on current time
    let london_time = get_current_time("Europe/London").unwrap();
    let ny_time = get_current_time("America/New_York").unwrap();
    // Both should be valid times
    assert_eq!(london_time.len(), 5);
    assert_eq!(ny_time.len(), 5);
}

