use base64::Engine;
use std::{fs, io::Write, ops::Add};
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tmp_test().await?;
    Ok(())
}

fn get_mock_invoice() -> Result<invoice_template::InvoiceData, Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./json-schema/mock.json")?;
    let data: invoice_template::InvoiceData = serde_json::from_str(file.as_str()).unwrap();
    Ok(data)
}

async fn tmp_test() -> Result<(), Box<dyn std::error::Error>> {
    let style_content = invoice_template::get_style_str().unwrap();
    let data: invoice_template::InvoiceData = get_mock_invoice().unwrap();
    let html_content = invoice_template::render_to_str(data).await?;
    let encoded = base64::engine::general_purpose::STANDARD_NO_PAD
        .encode(html_content.add(style_content.as_str()));
    let url = "data:text/html;base64,".to_owned() + encoded.as_str();
    let pdf = pdf_loader::pdf_scrapper(url.as_str()).await?;
    let mut file = fs::File::create("./test.pdf")?;
    file.write_all(&pdf)?;
    Ok(())
}
