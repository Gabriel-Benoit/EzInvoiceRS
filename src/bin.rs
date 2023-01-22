use async_std;
use ez_invoice::{generate_invoice, InvoiceDataJson};
use std::{fs, io::Write};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_mock_invoice("./json-schema/mock.json")?;
    let pdf = generate_invoice(data).await;
    let mut file = fs::File::create("./invoice.pdf")?;
    file.write_all(&pdf)?;
    Ok(())
}

fn get_mock_invoice(path: &str) -> Result<InvoiceDataJson, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(path)?;
    let data: InvoiceDataJson = serde_json::from_str(file.as_str())?;
    Ok(data)
}
