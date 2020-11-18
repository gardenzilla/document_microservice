use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    payee_name: String,
    payee_address: String,
    receiver_name: String,
    receiver_address: String,
    date: String,
    amount: i32,
}

impl Data {
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

#[derive(Serialize, Deserialize)]
pub struct CashIn {
    id: u32,
    data: Option<Data>,
}

impl crate::document::DocumentProvider<Data> for CashIn {
    fn init(id: u32) -> Result<Self, crate::document::DocumentError> {
        Ok(CashIn { id, data: None })
    }

    fn set_data(mut self, data: Data) -> Result<Self, crate::document::DocumentError> {
        self.data = Some(data);
        Ok(self)
    }

    fn get_data(&self) -> Option<Data> {
        self.data.clone()
    }

    fn get_location(&self) -> &'static str {
        "cash_in"
    }
    fn get_id(&self) -> u32 {
        self.id
    }
    fn get_prefix(&self) -> &'static str {
        "bga"
    }
}
