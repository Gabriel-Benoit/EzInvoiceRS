use async_std::stream::StreamExt;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::PrintToPdfParams,
};

pub async fn pdf_scrapper(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;
    let _handle = async_std::task::spawn(async move {
        let mut finish = false;
        while !finish {
            let _ = handler.next().await.unwrap_or_else(|| {
                finish = true;
                Ok(())
            });
        }
    });
    let page = browser.new_page(url).await?;
    let params = PrintToPdfParams {
        landscape: None,
        display_header_footer: Some(false),
        print_background: Some(true),
        scale: None,
        paper_width: None,
        paper_height: None,
        margin_top: Some(1.38),
        margin_bottom: Some(2.3),
        margin_left: Some(1.4),
        margin_right: Some(1.4),
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: Some(true),
        transfer_mode: None,
    };

    let pdf = page.pdf(params).await?;
    browser.close().await?;
    Ok(pdf)
}
