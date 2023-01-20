pub mod template;

use schemafy;
use serde::{Deserialize, Serialize};
schemafy::schemafy!(
    root: InvoiceData
    "./src/json-schema/schema.json");
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("./src/json-schema/mock.json")?;
    let nested: InvoiceData = serde_json::from_str(data.as_str())?;
    println!("{nested:?}");
    Ok(())
}
