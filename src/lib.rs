pub mod invoice_template;
pub use invoice_template::types::*;
use invoice_template::{get_style_str, render_to_str};

mod pdf_loader;

use base64::Engine;
use std::ops::Add;

/// Generates an invoice as a PDF file in a vector of bytes with respect to the given invoice data
pub async fn generate_invoice(data: InvoiceDataJson) -> Vec<u8> {
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

    // Using a headless chrome to print HTML as a pdf
    pdf_loader::pdf_scrapper(url.as_str())
        .await
        .unwrap_or_else(|e| {
            panic!("Couldn't convert file into PDF format. This issue was caused by the following error: {e}");
        })
}
