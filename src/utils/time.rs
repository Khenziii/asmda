use chrono::{DateTime, Local};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_current_formatted_date() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    datetime.format("%Y/%m/%d - %H:%M:%S").to_string()
}

pub fn system_time_to_str(system_time: SystemTime) -> String {
    let timestamp = system_time
        .duration_since(UNIX_EPOCH)
        .expect("Error! Passed date before the Unix epoch.")
        .as_secs();
    timestamp.to_string()
}

// `formatted_string` needs to be a string containing the amount of seconds since the Unix epoch.
pub fn str_to_system_time(formatted_string: String) -> SystemTime {
    let timestamp: u64 = formatted_string
        .parse()
        .expect("Failed to convert passed timestamp to integer! Is the string surely valid?");
    UNIX_EPOCH + Duration::from_secs(timestamp)
}

#[cfg(test)]
mod tests {
    mod system_time_to_str {
        use super::super::*;

        #[test]
        fn converts_correctly() {
            let timestamp = UNIX_EPOCH + Duration::from_secs(5);
            let timestamp_str = system_time_to_str(timestamp);

            assert_eq!(timestamp_str, "5");
        }

        #[test]
        #[should_panic]
        fn value_range_valid() {
            let timestamp = UNIX_EPOCH - Duration::from_secs(5);
            system_time_to_str(timestamp);
        }
    }

    mod str_to_system_time {
        use super::super::*;

        #[test]
        fn converts_correctly() {
            let timestamp = UNIX_EPOCH + Duration::from_secs(5);
            let timestamp_str = String::from("5");

            assert_eq!(timestamp, str_to_system_time(timestamp_str));
        }
    }
}
