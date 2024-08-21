pub mod server;
use async_std::stream::StreamExt;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::PrintToPdfParams,
};
use once_cell::sync::Lazy;
pub use server::rocket;
use tokio::sync::Mutex;

const PRINT_CONFIG: PrintToPdfParams = PrintToPdfParams {
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
    generate_document_outline: None,
    generate_tagged_pdf: None,
    header_template: None,
    footer_template: None,
    prefer_css_page_size: Some(true),
    transfer_mode: None,
};

static CHROME_INSTANCE: Lazy<Mutex<ChromeInstance>> =
    Lazy::new(|| async_std::task::block_on(async { Mutex::new(ChromeInstance::new().await) }));

pub struct ChromeInstance {
    pub browser: Browser,
}

/// Initializes a static chrome instance to be used by the server,
/// otherwise the lazy evaluation will be applied when the first request is made.
#[allow(unused_must_use)]
pub async fn init_chrome_instance() {
    CHROME_INSTANCE.lock().await;
}

impl ChromeInstance {
    pub async fn new() -> Self {
        let (browser, mut handler) =
            Browser::launch(BrowserConfig::builder().no_sandbox().build().unwrap())
                .await
                .unwrap_or_else(|e| panic!("Failed to launch browser: {:?}", e));
        let _handle = async_std::task::spawn(async move {
            let mut finish = false;
            while !finish {
                let _ = handler.next().await.unwrap_or_else(|| {
                    finish = true;
                    Ok(())
                });
            }
        });
        Self { browser }
    }
}

/// Generates a PDF file in a vector of bytes using a static headless chrome instance with respect to the given url
pub async fn static_pdf_scrapper(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Acquire chrome instance lock
    let chrome = CHROME_INSTANCE.lock().await;

    // Print procedure
    let page = chrome.browser.new_page(url).await?;
    let pdf = page.pdf(PRINT_CONFIG).await?;
    page.close().await?;

    Ok(pdf)
}

/// Generates a PDF file in a vector of bytes using a local headless chrome instance with respect to the given url
pub async fn pdf_scrapper(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Launch browser
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

    // Print procedure
    let page = browser.new_page(url).await?;
    let pdf = page.pdf(PRINT_CONFIG).await?;
    browser.close().await?;

    Ok(pdf)
}
