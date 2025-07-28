pub mod utils {
    use chrono::Utc;

pub fn get_timestamp() -> Option<i64> {
    let current_time = Utc::now().timestamp();
    Some(current_time)
}
}