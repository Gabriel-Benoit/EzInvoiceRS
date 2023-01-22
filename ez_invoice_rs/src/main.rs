use base64::Engine;
use std::{fs, io::Write, ops::Add};
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: invoice_template::InvoiceDataJson = get_mock_invoice().unwrap();
    let pdf = generate_invoice(data).await.unwrap();
    let mut file = fs::File::create("./invoice.pdf")?;
    file.write_all(&pdf)?;
    Ok(())
}

fn get_mock_invoice() -> Result<invoice_template::InvoiceDataJson, Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./json-schema/mock.json")?;
    let data: invoice_template::InvoiceDataJson = serde_json::from_str(file.as_str()).unwrap();
    Ok(data)
}

/// Generates an invoice as a PDF file in a vector of bytes with respect to the given invoice data
pub async fn generate_invoice(
    data: invoice_template::InvoiceDataJson,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Getting CSS file as a string
    let style_content = invoice_template::get_style_str().unwrap_or_else(|e|{
        println!("Couldn't load style file. Ignoring style. This issue was caused by the following error: {e}");
        String::default()
    });

    // Getting HTML file as a string
    let html_content = invoice_template::render_to_str(data)
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Couldn't render HTML content. This issue was caused by the following error: {e}"
            );
        });

    // Encoding HTML + CSS in base64
    let encoded = base64::engine::general_purpose::STANDARD_NO_PAD
        .encode(html_content.add(style_content.as_str()));

    // Creating data URL
    let url = "data:text/html;base64,".to_owned() + encoded.as_str();

    // Using a headless chrome to print HTML as a pdf
    let pdf = pdf_loader::pdf_scrapper(url.as_str())
        .await
        .unwrap_or_else(|e| {
            panic!("Couldn't convert file into PDF format. This issue was caused by the following error: {e}");
        });

    Ok(pdf)
}
