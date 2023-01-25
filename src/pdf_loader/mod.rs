pub mod server;
use async_std::stream::StreamExt;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::PrintToPdfParams,
};
use once_cell::sync::Lazy;
pub use server::rocket;
use tokio::sync::{Mutex, MutexGuard};

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
    ignore_invalid_page_ranges: None,
    header_template: None,
    footer_template: None,
    prefer_css_page_size: Some(true),
    transfer_mode: None,
};

static mut CHROME_INSTANCE: Lazy<Mutex<Option<ChromeInstance>>> = Lazy::new(|| {
    async_std::task::block_on(async {
        let chr = Some(ChromeInstance::new().await);
        Mutex::new(chr)
    })
});

async fn get_chrome_instance() -> MutexGuard<'static, Option<ChromeInstance>> {
    unsafe {
        let mut guard = CHROME_INSTANCE.lock().await;
        if guard.is_none() {
            *guard = Some(ChromeInstance::new().await);
        }
        guard
    }
}

pub struct ChromeInstance {
    pub browser: Browser,
}

impl ChromeInstance {
    pub async fn new() -> Self {
        let (browser, mut handler) = Browser::launch(BrowserConfig::builder().build().unwrap())
            .await
            .unwrap();
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
    let mut chrome_guard = get_chrome_instance().await;
    let chrome = chrome_guard.as_mut().unwrap();

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
