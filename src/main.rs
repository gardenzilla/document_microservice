use std::error::Error;

mod document;
mod templates;

use document::*;

fn main() -> Result<(), Box<dyn Error>> {
    let cashin = templates::cash_in::CashIn::init(18)
        .unwrap()
        .set_data(templates::cash_in::Data::new())
        .unwrap()
        .create_document()
        .unwrap()
        .get_document_base64()
        .unwrap();
    println!("Base64 is => {}", cashin);
    Ok(())
}
