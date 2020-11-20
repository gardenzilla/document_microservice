use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CashIn {
    payee_name: String,
    payee_address: String,
    receiver_name: String,
    receiver_address: String,
    date: String,
    amount: i32,
}

impl CashIn {
    pub fn new(
        payee_name: String,
        payee_address: String,
        receiver_name: String,
        receiver_address: String,
        date: String,
        amount: i32,
    ) -> Self {
        Self {
            payee_name,
            payee_address,
            receiver_name,
            receiver_address,
            date,
            amount,
        }
    }
}

impl crate::document::DocumentProvider for CashIn {
    fn get_location(&self) -> &'static str {
        "cash_in"
    }
}
