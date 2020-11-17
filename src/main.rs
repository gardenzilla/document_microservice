use std::error::Error;

mod document;
mod templates;

fn main() -> Result<(), Box<dyn Error>> {
    match document::create_document(2, document::DocumentKind::CashIn, ()) {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error {:?}", e),
    }
    Ok(())
}
