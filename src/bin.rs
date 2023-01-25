use ez_invoice::pdf_loader::server::rocket;
use ez_invoice::{generate_invoice, InvoiceDataJson};
use std::{fs, io::Write};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Static generation tests
    /*
    let data = get_mock_invoice("./json-schema/mock.json")?;
    let pdf = generate_invoice(&data).await?;
    write_pdf_to_file(pdf, None)?;
    */

    // Dynamic generation
    // This call blocks the current thread
    start_server().await.ok();

    Ok(())
}

async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    rocket().ignite().await.unwrap().launch().await.ok();
    Ok(())
}

fn get_mock_invoice(path: &str) -> Result<InvoiceDataJson, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(path)?;
    let data: InvoiceDataJson = serde_json::from_str(file.as_str())?;
    Ok(data)
}

fn write_pdf_to_file(pdf: Vec<u8>, path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.unwrap_or("./invoice.pdf");
    let mut file = fs::File::create(path)?;
    file.write_all(&pdf)?;
    Ok(())
}
