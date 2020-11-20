use std::error::Error;

mod document;
mod templates;

use chrono::prelude::*;
use document::*;

fn main() -> Result<(), Box<dyn Error>> {
    let provider = document::Provider::init("GRDN", templates::cash_in::CashIn::default());

    let data_object = templates::cash_in::CashIn::new(
        "Peter Mezei".into(),
        "4551 Nyíregyháza, Mogyorós utca 36.".into(),
        "Mezeiné B. Krisztina".into(),
        "4551 Nyíregyháza, Mogyorós utca 36.".into(),
        "2020-11-18 17:29".into(),
        75000,
    );

    let data_str = serde_json::to_string(&data_object).unwrap();

    let id = provider.create_document(&data_str).unwrap();

    println!("ID is => {}", id.to_string());

    println!(
        "Base 64 is => {}",
        provider.get_document_base64("GRDN-2020-1").unwrap()
    );

    Ok(())
}

mod v2 {
    use std::sync::Mutex;
    pub struct Providers {
        cash_in: (),
        cash_out: (),
        inventory_log: (),
        procurement: (),
    }

    pub struct DocumentService {
        providers: Mutex<Providers>,
    }
}
