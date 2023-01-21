use futures::StreamExt;

use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::PrintToPdfParams,
};

pub async fn pdf_scrapper(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;
    let handle = async_std::task::spawn(async move {
        let mut finish = false;
        while !finish {
            let _ = handler.next().await.unwrap_or_else(|| {
                finish = true;
                Ok(())
            });
        }
    });
    let page = browser.new_page(url).await?;
    let params = PrintToPdfParams::default();
    let pdf = page.pdf(params).await?;
    browser.close().await?;
    Ok(pdf)
}
