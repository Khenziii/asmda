use uuid::Uuid;

pub fn get_random() -> String {
    Uuid::new_v4().to_string()
}
