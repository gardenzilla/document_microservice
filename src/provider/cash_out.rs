use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CashOut {
    name: String,
}

impl crate::document::DocumentProvider for CashOut {
    fn get_location(&self) -> &'static str {
        "cash_out"
    }
}
