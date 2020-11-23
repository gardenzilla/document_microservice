extern crate handlebars;
extern crate serde_json;

use std::error::Error;

mod document;
pub mod number_text;
mod provider;

use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let provider = document::Provider::init("GZCIA", provider::cash_in::CashIn::default());
    let provider2 = document::Provider::init("GRDZ", provider::cash_out::CashOut::default());

    let data_object = provider::cash_in::CashIn::new(
        "Peter Mezei".into(),
        "4551 Nyíregyháza, Mogyorós utca 36.".into(),
        "Mezeiné B. Krisztina".into(),
        "4551 Nyíregyháza, Mogyorós utca 36.".into(),
        "2020-11-18 17:29".into(),
        1_127_945,
    );

    let data_str = serde_json::to_string(&data_object).unwrap();

    let now = Instant::now();

    let id = provider.create_document(&data_str).unwrap();
    println!("ID is => {}", id.to_string());

    println!("Time elapsed: {:?}", now.elapsed());

    let now = Instant::now();

    let _ = provider2
        .create_document("{\"name\":\"Peter Mezei\"}")
        .unwrap();

    println!("Time elapsed: {:?}", now.elapsed());

    // let now = Instant::now();

    // let base_64 = provider.get_document_base64("GRDB-2020-1").unwrap();
    // println!("base64 len is {}", base_64.len());

    // println!("Time elapsed: {:?}", now.elapsed());

    // println!(
    //     "base64 is: {}",
    //     provider.get_document_base64("GRDB-2020-1").unwrap()
    // );

    // println!(
    //     "Base 64 is => {}",
    //     provider.get_document_base64("GRDN-2020-1").unwrap()
    // );

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
