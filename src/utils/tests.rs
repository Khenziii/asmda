#[cfg(test)]
pub fn is_test_environment() -> bool {
    true
}

#[cfg(not(test))]
pub fn is_test_environment() -> bool {
    false
}
