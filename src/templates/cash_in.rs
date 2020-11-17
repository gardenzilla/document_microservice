use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Data {
    payee_name: String,
    payee_address: String,
    receiver_name: String,
    receiver_address: String,
    date: DateTime<Utc>,
    amount: i32,
}
