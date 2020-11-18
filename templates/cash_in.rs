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

    fn get_data(&self) -> Option<&Data> {
        self.data.as_ref()
    }

    fn get_location(&self) -> &'static str {
        "cash_in"
    }
    fn get_id(&self) -> u32 {
        self.id
    }
}
