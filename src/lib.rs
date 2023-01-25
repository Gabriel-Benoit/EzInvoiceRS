pub mod invoice_template;
pub use invoice_template::types::*;
pub mod pdf_loader;
use invoice_template::{get_style_str, render_to_str};
pub use pdf_loader::server::rocket;

use base64::Engine;
use std::ops::Add;

/// Generates an invoice as a PDF file in a vector of bytes with respect to the given invoice data
pub async fn generate_invoice(
    data: &InvoiceDataJson,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = generate_url(data).await;
    // Using a headless chrome to print HTML as a pdf
    let pdf = pdf_loader::pdf_scrapper(url.as_str()).await?;
    Ok(pdf)
}

/// Generates an invoice as a PDF file in a vector of bytes with respect to the given invoice data
/// The process is static, meaning that the invoice is generated with a static instance of a headless browser
pub async fn static_generate_invoice(
    data: &InvoiceDataJson,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = generate_url(data).await;
    // Using a static headless chrome instance to print HTML as a pdf
    let pdf = pdf_loader::static_pdf_scrapper(url.as_str()).await?;
    Ok(pdf)
}

/// Generates an invoice as a data URL with respect to the given invoice data
async fn generate_url(data: &InvoiceDataJson) -> String {
    // Getting CSS file as a string
    let style_content = get_style_str().unwrap_or_else(|e|{
        println!("Couldn't load style file. Ignoring style. This issue was caused by the following error: {e}");
        String::default()
    });

    // Getting HTML file as a string
    let html_content = render_to_str(data).await;

    // Encoding HTML + CSS in base64
    let encoded = base64::engine::general_purpose::STANDARD_NO_PAD
        .encode(html_content.add(style_content.as_str()));

    // Creating data URL
    let url = "data:text/html;base64,".to_owned() + encoded.as_str();
    url
}
