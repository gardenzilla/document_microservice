use std::error::Error;

mod document;
mod templates;

use chrono::prelude::*;
use document::*;

fn main() -> Result<(), Box<dyn Error>> {
    let cashin = templates::cash_in::CashIn::init(29)
        .unwrap()
        .set_data(templates::cash_in::Data::new(
            "Peter Mezei".into(),
            "4551 Nyíregyháza, Mogyorós utca 36.".into(),
            "Mezeiné B. Krisztina".into(),
            "4551 Nyíregyháza, Mogyorós utca 36.".into(),
            "2020-11-18 17:29".into(),
            75000,
        ))
        .unwrap()
        .create_document()
        .unwrap()
        .get_document_base64()
        .unwrap();
    println!("Base64 is => {}", cashin);
    Ok(())
}
