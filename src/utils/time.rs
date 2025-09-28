use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
