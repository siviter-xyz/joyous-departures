use chrono::{DateTime, Local};
use chrono_tz::Tz;

use crate::error::GoodbyeError;

/// Generate current date string in YYYY-MM-DD format for the given timezone
pub fn get_current_date(timezone: &str) -> Result<String, GoodbyeError> {
    let tz: Tz = timezone
        .parse()
        .map_err(|_| GoodbyeError::InvalidTimezoneError(timezone.to_string()))?;

    let now: DateTime<Tz> = Local::now().with_timezone(&tz);
    Ok(now.format("%Y-%m-%d").to_string())
}

/// Generate current time string in HH:MM format for the given timezone
pub fn get_current_time(timezone: &str) -> Result<String, GoodbyeError> {
    let tz: Tz = timezone
        .parse()
        .map_err(|_| GoodbyeError::InvalidTimezoneError(timezone.to_string()))?;

    let now: DateTime<Tz> = Local::now().with_timezone(&tz);
    Ok(now.format("%H:%M").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_date() {
        let date = get_current_date("Europe/London").unwrap();
        assert_eq!(date.len(), 10); // YYYY-MM-DD
        assert!(date.contains('-'));
    }

    #[test]
    fn test_get_current_time() {
        let time = get_current_time("Europe/London").unwrap();
        assert_eq!(time.len(), 5); // HH:MM
        assert!(time.contains(':'));
    }

    #[test]
    fn test_invalid_timezone() {
        assert!(get_current_date("Invalid/Timezone").is_err());
        assert!(get_current_time("Invalid/Timezone").is_err());
    }
}
