use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Status {
    pub message: String,
    pub system_running: bool,
    pub timestamp: i64,
    pub valid_version: bool,
}
