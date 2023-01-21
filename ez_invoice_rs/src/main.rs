use base64::Engine;
use schemafy;
use serde::{Deserialize, Serialize};

schemafy::schemafy!(
    root: InvoiceData
    "../json-schema/schema.json");

use std::{fs, io::Write, ops::Add};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let style_content = invoice_template::get_style_str().unwrap();
    let html_content = invoice_template::render_to_str().await?;
    let encoded = base64::engine::general_purpose::STANDARD_NO_PAD
        .encode(html_content.add(style_content.as_str()));
    let url = "data:text/html;base64,".to_owned() + encoded.as_str();
    let pdf = pdf_loader::pdf_scrapper(url.as_str()).await?;
    let mut file = fs::File::create("./test.pdf")?;
    file.write_all(&pdf)?;
    Ok(())
}

//pub fn generate_pdf_invoice
